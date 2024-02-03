use std::str::FromStr;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
	extract::Path,
	http::StatusCode,
	response::Redirect,
	routing::{get, post},
	Router,
};
use axum_extra::extract::Form;
use chrono::{DateTime, Duration, Utc};
use sea_orm::{
	ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel,
	QueryFilter,
};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::{app::App, db, web::auth};

use super::{
	auth::{AuthResult, RequireAuth, RequireSysop},
	meta::MessagePage,
	NotFoundPage, WebError, WebResult,
};

#[derive(Template)]
#[template(path = "user.html")]
struct InfoPage {
	auth: AuthResult,
	user: db::user::Model,
	is_self: bool,
	is_blocked: bool,
	is_permanent_block: bool,
}

pub fn new_router() -> Router {
	Router::new()
		.route(
			"/",
			get(|auth: RequireAuth| async move {
				Redirect::temporary(&format!("/user/{}", auth.info().id))
			}),
		)
		.route("/:id", get(get_user_handler))
		.route("/:id/op", post(op_handler))
		.route("/:id/deop", post(deop_handler))
		.route("/:id/reset_salt", post(reset_salt_handler))
		.route("/:id/block", post(block_handler))
		.route("/:id/block/permanent", post(permanent_block_handler))
		.route("/:id/unblock", post(unblock_handler))
		.route("/:id/lang", post(set_language_handler))
}

fn user_or_else(
	auth: &AuthResult,
	user: Option<db::user::Model>,
) -> Result<db::user::Model, WebError> {
	match user {
		Some(user) => Ok(user),
		None => Err(WebError::Response(
			NotFoundPage {
				auth: auth.to_owned(),
				message: "user not found",
			}
			.into_response(),
		)),
	}
}

async fn get_user_handler(
	auth: AuthResult,
	Path(id): Path<String>,
) -> WebResult {
	if let Ok(id) = Uuid::from_str(&id) {
		let user = user_or_else(
			&auth,
			db::user::Entity::find_by_id(id).one(&*db::get()).await?,
		)?;
		Ok(
			Redirect::temporary(&format!("/user/{}", user.name))
				.into_response(),
		)
	} else if let Some(user) = db::user::Entity::find()
		.filter(db::user::Column::Name.eq(id))
		.one(&*db::get())
		.await?
	{
		let is_self = if let Some(auth) = &auth.0
			&& auth.id == user.id
		{
			true
		} else {
			false
		};
		let is_blocked = user.blocked.is_some();
		let is_permanent_block = user
			.blocked
			.map(|t| (t - Utc::now().naive_utc()).num_days() >= 3652)
			.unwrap_or(false);
		return Ok(InfoPage {
			auth,
			user,
			is_self,
			is_blocked,
			is_permanent_block,
		}
		.into_response());
	} else {
		return Ok(NotFoundPage {
			auth,
			message: "user with given name not found",
		}
		.into_response());
	}
}

async fn op_handler(
	RequireSysop(auth): RequireSysop,
	Path(id): Path<Uuid>,
) -> WebResult {
	let user = user_or_else(
		&auth,
		db::user::Entity::find_by_id(id).one(&*db::get()).await?,
	)?;
	info!(target = %user, user = %auth, "add user as bot-sysop");
	let mut user = user.into_active_model();
	user.sysop = ActiveValue::Set(true);
	let user = user.update(&*db::get()).await?;
	Ok(MessagePage {
		auth,
		title: "Assigned bot-sysop permission",
		message: &format!("{} is now an operator.", user.name),
		auto_return: false,
	}
	.into_response())
}

async fn deop_handler(
	RequireSysop(auth): RequireSysop,
	Path(id): Path<Uuid>,
) -> WebResult {
	let user = user_or_else(
		&auth,
		db::user::Entity::find_by_id(id).one(&*db::get()).await?,
	)?;
	info!(target = %user, user = %auth, "remove user from bot-sysop");
	let mut user = user.into_active_model();
	user.sysop = ActiveValue::Set(false);
	let user = user.update(&*db::get()).await?;
	Ok(MessagePage {
		auth,
		title: "Remove bot-sysop permission",
		message: &format!("{} is no longer an operator.", user.name),
		auto_return: false,
	}
	.into_response())
}

async fn reset_salt_handler(
	RequireAuth(auth): RequireAuth,
	Path(id): Path<Uuid>,
) -> WebResult {
	if !auth.0.as_ref().unwrap().sysop && auth.0.as_ref().unwrap().id != id {
		return Err(WebError::Response(
			(StatusCode::UNAUTHORIZED, "bot-sysop or login required")
				.into_response(),
		));
	}
	let user = user_or_else(
		&auth,
		db::user::Entity::find_by_id(id).one(&*db::get()).await?,
	)?;
	info!(target = %user, user = %auth, "reset user token-salt");
	let mut user = user.into_active_model();
	user.salt = ActiveValue::Set(auth::generate_salt());
	let user = user.update(&*db::get()).await?;
	App::get().login_lru.write().clear();
	Ok(MessagePage {
		auth,
		title: "Token-salt reseted",
		message: &format!(
			"All tokens for {} should be invalid now.",
			user.name
		),
		auto_return: false,
	}
	.into_response())
}

#[derive(Deserialize)]
struct BlockParams {
	#[serde(deserialize_with = "::duration_str::deserialize_duration_chrono")]
	time: Duration,
}

async fn block_handler(
	RequireSysop(auth): RequireSysop,
	Path(id): Path<Uuid>,
	Form(params): Form<BlockParams>,
) -> WebResult {
	let user = user_or_else(
		&auth,
		db::user::Entity::find_by_id(id).one(&*db::get()).await?,
	)?;
	let time = params.time;
	info!(target = %user, user = %auth, %time, "block user");
	let mut user = user.into_active_model();
	user.blocked = ActiveValue::Set(Some(Utc::now().naive_utc() + time));
	let user = user.update(&*db::get()).await?;
	App::get().login_lru.write().clear();
	Ok(MessagePage {
		auth,
		title: "User blocked",
		message: &format!("User {} is blocked for {}.", user.name, time),
		auto_return: true,
	}
	.into_response())
}

async fn permanent_block_handler(
	RequireSysop(auth): RequireSysop,
	Path(id): Path<Uuid>,
) -> WebResult {
	let user = user_or_else(
		&auth,
		db::user::Entity::find_by_id(id).one(&*db::get()).await?,
	)?;
	info!(target = %user, user = %auth, "permanently block user");
	let mut user = user.into_active_model();
	user.blocked = ActiveValue::Set(Some(DateTime::<Utc>::MAX_UTC.naive_utc()));
	let user = user.update(&*db::get()).await?;
	App::get().login_lru.write().clear();
	Ok(MessagePage {
		auth,
		title: "User permanently blocked",
		message: &format!("User {} is now permanently blocked.", user.name),
		auto_return: true,
	}
	.into_response())
}

async fn unblock_handler(
	RequireSysop(auth): RequireSysop,
	Path(id): Path<Uuid>,
) -> WebResult {
	let user = user_or_else(
		&auth,
		db::user::Entity::find_by_id(id).one(&*db::get()).await?,
	)?;
	info!(target = %user, user = %auth, "unblock user");
	let mut user = user.into_active_model();
	user.blocked = ActiveValue::Set(None);
	let user = user.update(&*db::get()).await?;
	App::get().login_lru.write().clear();
	Ok(MessagePage {
		auth,
		title: "User unblocked",
		message: &format!("User {} is unblocked.", user.name),
		auto_return: true,
	}
	.into_response())
}

#[derive(Deserialize)]
struct SetLangParams {
	lang: String,
}

async fn set_language_handler(
	RequireAuth(auth): RequireAuth,
	Path(id): Path<Uuid>,
	Form(params): Form<SetLangParams>,
) -> WebResult {
	if !auth.0.as_ref().unwrap().sysop && auth.0.as_ref().unwrap().id != id {
		return Err(WebError::Response(
			(StatusCode::UNAUTHORIZED, "bot-sysop or login required")
				.into_response(),
		));
	}
	let user = user_or_else(
		&auth,
		db::user::Entity::find_by_id(id).one(&*db::get()).await?,
	)?;
	let lang = params.lang;
	info!(target = %user, user = %auth, lang, "set user language");
	let mut user = user.into_active_model();
	user.language = ActiveValue::Set(lang.clone());
	user.update(&*db::get()).await?;
	App::get().login_lru.write().clear();
	Ok(MessagePage {
		auth,
		title: "Language set",
		message: &format!("Language has been set to {}", lang),
		auto_return: false,
	}
	.into_response())
}
