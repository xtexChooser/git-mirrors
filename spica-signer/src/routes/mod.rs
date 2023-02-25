use askama::Template;
use axum::{routing::get, Router};

pub async fn make_router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
