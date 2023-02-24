use axum::{response::Html, routing::get, Router};

pub async fn make_router() -> Router {
    Router::new().route(
        "/",
        get(|| async { Html(include_str!("../../index.html")) }),
    )
}
