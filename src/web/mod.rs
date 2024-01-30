use axum::{http::StatusCode, response::IntoResponse, Router};
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::app::App;

pub mod auth;
pub mod meta;
pub mod sysop;

pub async fn run_server() {
	let app = App::get();
	let addr = std::env::var("SPOCK_LISTEN").unwrap_or("0.0.0.0:3000".to_owned());

	let router = Router::new()
		.with_state(app.to_owned())
		.nest("/", meta::new_router())
		.nest("/auth", auth::new_router())
		.nest("/sysop", sysop::new_router());

	info!(addr, "Start tcp listener");
	let listener = TcpListener::bind(addr).await.unwrap();
	axum::serve(listener, router).await.unwrap();
}

type WebResult = Result<axum::response::Response, WebError>;

struct WebError(anyhow::Error);

impl IntoResponse for WebError {
	fn into_response(self) -> axum::response::Response {
		error!(err = %self.0, "error in request");
		(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Something went wrong: {}", self.0),
		)
			.into_response()
	}
}

impl<E> From<E> for WebError
where
	E: Into<anyhow::Error>,
{
	fn from(err: E) -> Self {
		Self(err.into())
	}
}
