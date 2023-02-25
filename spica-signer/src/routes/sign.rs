use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};

use crate::cert::get_cert;

pub async fn make_router() -> Router {
    Router::new().route("/:id/sign/csr", post(sign_csr))
}

async fn sign_csr(Path(id): Path<String>, csr: String) -> Result<impl IntoResponse, StatusCode> {
    match get_cert(&id) {
        Some(cert) => {
            // re-format
            let req = pem::encode(&pem::parse(csr).unwrap());
            Ok((
                [(CONTENT_TYPE, "application/x-x509-ca-cert")],
                req.to_owned(),
            ))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
