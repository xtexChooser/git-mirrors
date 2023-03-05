use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::Path, routing::get, Json, Router};
use reqwest::{header::CONTENT_TYPE, StatusCode};

use crate::{
    cert::{get_cert, get_certs},
    config::{get_config, Config},
};

pub mod auth;
pub mod nodeinfo;
pub mod sign;

pub async fn make_router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/config.html", get(config))
        .route("/certs", get(cert_list))
        .route("/:id", get(cert_probe))
        .route("/:id/text.txt", get(cert_text))
        .route("/:id/cert.crt", get(cert_crt))
        .merge(nodeinfo::make_router().await)
        .merge(sign::make_router().await)
}

async fn index() -> IndexTemplate {
    IndexTemplate {
        config: get_config(),
        show_config: false,
    }
}

async fn config() -> IndexTemplate {
    IndexTemplate {
        config: get_config(),
        show_config: true,
    }
}

#[derive(Template)]
#[template(path = "index.html.j2")]
struct IndexTemplate {
    config: &'static Config,
    show_config: bool,
}

async fn cert_list() -> Json<Vec<&'static String>> {
    Json(get_certs().keys().collect())
}

async fn cert_probe(Path(id): Path<String>) -> StatusCode {
    match get_cert(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

async fn cert_text(Path(id): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    match get_cert(&id) {
        Some(cert) => Ok(cert.text.as_str()),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn cert_crt(Path(id): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    match get_cert(&id) {
        Some(cert) => Ok((
            [(CONTENT_TYPE, "application/x-x509-ca-cert")],
            cert.cert_pem.as_str(),
        )),
        None => Err(StatusCode::NOT_FOUND),
    }
}
