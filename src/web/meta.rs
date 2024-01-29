use askama::Template;
use axum::{http::header::CONTENT_TYPE, routing::get, Router};

const MAIN_CSS: &str = grass::include!("styles/main.sass");

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage {}

pub fn new_router() -> Router {
	Router::new()
		.route("/", get(|| async { IndexPage {} }))
		.route(
			"/main.css",
			get(|| async { ([(CONTENT_TYPE, "text/css; charset=utf-8")], MAIN_CSS) }),
		)
}
