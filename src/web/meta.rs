use askama::Template;
use axum::{http::header::CONTENT_TYPE, routing::get, Router};

use super::auth::AuthResult;

const MAIN_CSS: &str = grass::include!("styles/main.sass");

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {
	auth: AuthResult,
}

#[derive(Template)]
#[template(path = "message.html")]
pub struct MessagePage<'a> {
	pub auth: AuthResult,
	pub title: &'a str,
	pub message: &'a str,
	pub auto_return: bool,
}

const MESSAGE_AUTO_RETURN_JS: &str =
	include_str!("../../assets/message-auto-return.js");

pub fn new_router() -> Router {
	Router::new()
		.route("/", get(|auth: AuthResult| async { IndexPage { auth } }))
		.route(
			"/main.css",
			get(|| async {
				([(CONTENT_TYPE, "text/css; charset=utf-8")], MAIN_CSS)
			}),
		)
		.route(
			"/message-auto-return.js",
			get(|| async {
				(
					[(CONTENT_TYPE, "text/javascript; charset=utf-8")],
					MESSAGE_AUTO_RETURN_JS,
				)
			}),
		)
}
