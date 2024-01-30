use askama::Template;
use axum::{http::header::CONTENT_TYPE, routing::get, Router};

use super::auth::AuthResult;

#[derive(Template)]
#[template(path = "index.html")]
struct SysopPage {
	auth: AuthResult,
}

pub fn new_router() -> Router {
	Router::new()
		.route("/", get(|auth: AuthResult| async { SysopPage { auth } }))
}
