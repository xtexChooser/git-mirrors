use std::{
	collections::HashMap, convert::Infallible, env, fmt::Display, str::FromStr, sync::LazyLock,
};

use anyhow::{bail, Result};
use askama::{filters::urlencode, Template};
use askama_axum::IntoResponse;
use axum::{
	async_trait,
	extract::{FromRequestParts, Query},
	http::{header, request::Parts, StatusCode},
	response::Redirect,
	routing::get,
	Router,
};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::Utc;
use rand::{distributions::DistString, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sea_orm::{
	ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tracing::{error, info, info_span, Instrument};
use uuid::Uuid;

use crate::{app::App, db};

use super::{meta::MessagePage, WebResult};

pub fn generate_salt() -> String {
	let mut rng = ChaCha20Rng::from_entropy();
	rand::distributions::Alphanumeric.sample_string(&mut rng, 64)
}

pub fn generate_token(salt: &str) -> String {
	let mut rng = ChaCha20Rng::from_entropy();
	let random = rand::distributions::Alphanumeric.sample_string(&mut rng, 16);
	format!("{}:{}", random, hash_token(salt, &random))
}

fn hash_token(salt: &str, random: &str) -> String {
	hex::encode(Sha256::digest(format!("{}{}", random, salt).as_bytes()))
}

pub fn validate_token(salt: &str, token: &str) -> bool {
	if let Some((random, hash)) = token.split_once(':') {
		hash_token(salt, random) == hash
	} else {
		false
	}
}

pub fn new_router() -> Router {
	Router::new().route("/", get(auth_handler)).route(
		"/logout",
		get(|| async {
			(
				[(
					header::SET_COOKIE,
					"spock_token=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT"
						.to_string(),
				)],
				MessagePage {
					auth: AuthResult(None),
					title: "Logout Succeeded",
					message: "You are now logged out.",
					auto_return: true,
				},
			)
		}),
	)
}

#[derive(Template)]
#[template(path = "auth_success.html")]
struct AuthSuccessPage {
	auth: AuthResult,
}

#[derive(Deserialize)]
struct AuthParams {
	code: Option<String>,
}

static OAUTH_ID: LazyLock<String> =
	LazyLock::new(|| env::var("SPOCK_OAUTH_ID").expect("SPOCK_OAUTH_ID is missing"));
static OAUTH_SECRET: LazyLock<String> =
	LazyLock::new(|| env::var("SPOCK_OAUTH_SECRET").expect("SPOCK_OAUTH_SECRET is missing"));
static OAUTH_URL: LazyLock<String> = LazyLock::new(|| {
	env::var("SPOCK_OAUTH_REDIRECT_URI").expect("SPOCK_OAUTH_REDIRECT_URI is missing")
});
static OAUTH_URL_ENCODED: LazyLock<String> = LazyLock::new(|| {
	urlencode(OAUTH_URL.as_str()).expect("SPOCK_OAUTH_REDIRECT_URI can not be URL-encoded")
});
static OAUTH_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(Default::default);
static OAUTH_SYSOP: LazyLock<String> =
	LazyLock::new(|| env::var("SPOCK_OAUTH_SYSOP").expect("SPOCK_OAUTH_SYSOP is missing"));

#[derive(Debug, Serialize, Deserialize)]
struct MrTokenResponse {
	pub access_token: String,
	pub token_type: String,
	pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct MrUserResponse {
	pub id: String,
	pub username: String,
	pub name: String,
}

async fn auth_handler(auth: AuthResult, Query(params): Query<AuthParams>) -> WebResult {
	if let Some(code) = params.code {
		let token = async {
			let resp = OAUTH_CLIENT
				.post("https://api.modrinth.com/_internal/oauth/token")
				.form(&HashMap::from([
					("grant_type", "authorization_code"),
					("code", &code),
					("redirect_uri", &OAUTH_URL),
					("client_id", &OAUTH_ID),
				]))
				.header(reqwest::header::AUTHORIZATION, OAUTH_SECRET.as_str())
				.send()
				.await?
				.error_for_status()?
				.json::<MrTokenResponse>()
				.await?;
			if resp.token_type != "Bearer" {
				bail!("MR oauth/token responded with non-Bearer token_type");
			}
			let resp = OAUTH_CLIENT
				.get("https://api.modrinth.com/v3/user")
				.header(reqwest::header::AUTHORIZATION, &resp.access_token)
				.send()
				.await?
				.error_for_status()?
				.json::<MrUserResponse>()
				.await?;
			let mrid = resp.id;
			let user = db::user::Entity::find()
				.filter(db::user::Column::ModrinthId.eq(&mrid))
				.one(&*db::get())
				.await?;
			let mut user = match user {
				Some(u) => u,
				None => {
					db::user::ActiveModel {
						id: ActiveValue::Set(Uuid::new_v4()),
						name: ActiveValue::Set(resp.username.clone()),
						salt: ActiveValue::Set(generate_salt()),
						modrinth_id: ActiveValue::Set(mrid.clone()),
						sysop: ActiveValue::Set(mrid == *OAUTH_SYSOP),
						blocked: ActiveValue::Set(None),
					}
					.insert(&*db::get())
					.await?
				}
			};
			if let Some(blocked) = user.blocked {
				if blocked <= Utc::now() {
					user = {
						let mut model = user.into_active_model();
						model.blocked = ActiveValue::Set(None);
						model.update(&*db::get()).await?
					};
				} else if !user.sysop {
					return Ok(Err(blocked));
				}
			}
			let token = generate_token(&user.salt);
			info!(%user, token, "generated token per login request");
			Ok(Ok(format!("{}:{}", user.id, token)))
		}
		.instrument(info_span!("handle_oauth", code))
		.await?;
		match token {
			Ok(token) => Ok((
				[(
					header::SET_COOKIE,
					Cookie::new("spock_token", &token).to_string(),
				)],
				AuthSuccessPage {
					auth: AuthResult(login(&token).await),
				},
			)
				.into_response()),
			Err(blocked) => Ok((
				StatusCode::FORBIDDEN,
				MessagePage {
					auth: AuthResult(None),
					title: "Login Blocked",
					message: &format!(
						"You are blocked until {}",
						blocked.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
					),
					auto_return: false,
				},
			)
				.into_response()),
		}
	} else if auth.0.is_some() {
		Ok(AuthSuccessPage { auth }.into_response())
	} else {
		Ok(Redirect::temporary(&format!("https://modrinth.com/auth/authorize?client_id={}&redirect_uri={}&scope=USER_READ+USER_READ_EMAIL", OAUTH_ID.as_str(), OAUTH_URL_ENCODED.as_str())).into_response())
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct AuthInfo {
	pub id: Uuid,
	pub name: String,
	pub sysop: bool,
}

impl From<db::user::Model> for AuthInfo {
	fn from(value: db::user::Model) -> Self {
		Self {
			id: value.id,
			name: value.name,
			sysop: value.sysop,
		}
	}
}

pub async fn login(token: &str) -> Option<AuthInfo> {
	if let Some((user, token)) = token.split_once(':')
		&& let Ok(user) = Uuid::from_str(user)
		&& let Ok(Some(user)) = db::user::Entity::find_by_id(user).one(&*db::get()).await
		&& validate_token(&user.salt, token)
	{
		if let Some(blocked) = user.blocked {
			if blocked <= Utc::now() {
				let mut user = user.into_active_model();
				user.blocked = ActiveValue::Set(None);
				match user.update(&*db::get()).await {
					Ok(user) => return Some(user.into()),
					Err(error) => {
						error!(%error, "failed to remove block record for user");
						return None;
					}
				}
			} else if !user.sysop {
				return None;
			}
		}
		Some(user.into())
	} else {
		None
	}
}

#[derive(Debug, Clone)]
pub struct AuthResult(pub Option<AuthInfo>);

#[async_trait]
impl<S> FromRequestParts<S> for AuthResult
where
	S: Send + Sync,
{
	type Rejection = Infallible;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		if let Some(token) = CookieJar::from_request_parts(parts, state)
			.await
			.unwrap()
			.get("spock_token")
			.to_owned()
		{
			if let Some(result) = App::get().login_lru.write().get(token.value()) {
				return Ok(result.to_owned());
			}
			let result = AuthResult(login(token.value()).await);
			App::get()
				.login_lru
				.write()
				.put(token.value().to_owned(), result.clone());
			Ok(result)
		} else {
			Ok(AuthResult(None))
		}
	}
}

impl Display for AuthResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.0 {
			None => f.write_str("(anon)"),
			Some(auth) => f.write_str(auth.id.to_string().as_str()),
		}
	}
}

pub struct RequireAuth(pub AuthResult);

impl RequireAuth {
	pub fn info(&self) -> &AuthInfo {
		self.0 .0.as_ref().unwrap()
	}
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let AuthResult(auth) = AuthResult::from_request_parts(parts, state).await.unwrap();
		if let Some(auth) = auth {
			Ok(RequireAuth(AuthResult(Some(auth))))
		} else {
			Err((StatusCode::UNAUTHORIZED, "login required"))
		}
	}
}

pub struct RequireSysop(pub AuthResult);

impl RequireSysop {
	pub fn info(&self) -> &AuthInfo {
		self.0 .0.as_ref().unwrap()
	}
}

#[async_trait]
impl<S> FromRequestParts<S> for RequireSysop
where
	S: Send + Sync,
{
	type Rejection = (StatusCode, &'static str);

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let auth = RequireAuth::from_request_parts(parts, state).await?;
		if auth.info().sysop {
			Ok(RequireSysop(AuthResult(auth.0 .0)))
		} else {
			Err((StatusCode::UNAUTHORIZED, "bot-sysop permission required"))
		}
	}
}
