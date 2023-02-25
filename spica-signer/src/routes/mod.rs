use askama::Template;
use axum::{routing::get, Router};

use crate::cert::get_certs;

pub async fn make_router() -> Router {
    Router::new().route("/", get(index))
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        certs: get_certs().keys().collect(),
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    certs: Vec<&'static String>,
}
