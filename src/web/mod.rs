use axum::{http::StatusCode, response::IntoResponse, Router};
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::{app::App, config};

use self::{auth::AuthResult, meta::MessagePage};

pub mod auth;
pub mod i18n;
pub mod page;
pub mod meta;
pub mod sysop;
pub mod user;

config!(LISTEN, str, "0.0.0.0:3000");

pub async fn run_server() {
	let app = App::get();
	let addr = *CONFIG_LISTEN;

	i18n::init().expect("failed to init i18n");

	let router = Router::new()
		.with_state(app.to_owned())
		.nest("/", meta::new_router())
		.nest("/auth", auth::new_router())
		.nest("/sysop", sysop::new_router())
		.nest("/user", user::new_router())
		.nest("/page", page::new_router());

	info!(addr, "Start tcp listener");
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, router).await.unwrap();
}

type WebResult = Result<axum::response::Response, WebError>;

pub enum WebError {
	Error(anyhow::Error),
	Response(axum::response::Response),
}

impl IntoResponse for WebError {
	fn into_response(self) -> axum::response::Response {
		match self {
			WebError::Error(error) => {
				error!(%error, "error in request");
				(
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Something went wrong: {}", error),
				)
					.into_response()
			}
			WebError::Response(response) => response,
		}
	}
}

impl<E> From<E> for WebError
where
	E: Into<anyhow::Error>,
{
	fn from(err: E) -> Self {
		Self::Error(err.into())
	}
}
pub struct NotFoundMessage(pub String);

impl IntoResponse for NotFoundMessage {
	fn into_response(self) -> axum::response::Response {
		(StatusCode::NOT_FOUND, self.0).into_response()
	}
}

pub struct NotFoundPage<'a> {
	pub auth: AuthResult,
	pub message: &'a str,
}

impl<'a> IntoResponse for NotFoundPage<'a> {
	fn into_response(self) -> axum::response::Response {
		(
			StatusCode::NOT_FOUND,
			MessagePage {
				auth: self.auth,
				title: "Not Found",
				message: self.message,
				auto_return: false,
			},
		)
			.into_response()
	}
}
