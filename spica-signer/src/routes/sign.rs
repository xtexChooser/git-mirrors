use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};

use crate::{cert::get_cert, csr::CertReq};

pub async fn make_router() -> Router {
    Router::new().route("/:id/sign/csr", post(sign_csr))
}

async fn sign_csr(Path(id): Path<String>, csr: String) -> Result<impl IntoResponse, StatusCode> {
    match get_cert(&id) {
        Some(cert) => {
            // re-format
            let req_pem = &pem::parse(csr).unwrap();
            //let req = CertReq::from_csr(req_pem)
            Ok((
                [(CONTENT_TYPE, "application/x-x509-ca-cert")],
                pem::encode(req_pem).to_owned(),
            ))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
