use std::{collections::HashMap, time::Duration};

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};

use crate::{acl::ACLRule, cert::get_cert, csr::CertReq};

pub async fn make_router() -> Router {
    Router::new().route("/:id/sign/csr", post(sign_csr))
}

async fn sign_csr(Path(id): Path<String>, csr: String) -> impl IntoResponse {
    match get_cert(&id) {
        Some(cert) => {
            // re-format
            let req_pem = match pem::parse(csr) {
                Ok(pem) => pem,
                Err(e) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        format!("failed to parse csr pem: {}", e.to_string()),
                    )
                        .into_response()
                }
            };
            let acl = ACLRule {
                certs: vec![],
                max_expire: Duration::from_secs(60 * 60 * 24 * 15),
                allowed_san_dns: vec![".*$".to_owned()],
                can_custom_serial: false,
                openssl_opt: HashMap::from([(
                    "basicConstraints".to_owned(),
                    "CA:FALSE".to_owned(),
                )]),
                prefer_hash: Some("sha256".to_owned()),
            };
            let req = CertReq::from_csr(&req_pem, None, None, acl, None);
            let req = match req {
                Ok(req) => req,
                Err(e) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        format!(
                            "failed to construct certification request from csr: {}",
                            e.to_string()
                        ),
                    )
                        .into_response()
                }
            };
            /*(
                StatusCode::OK,
                [(CONTENT_TYPE, "application/x-x509-ca-cert")],
                pem::encode(&req_pem).to_owned(),
            )
                .into_response()*/
            (StatusCode::OK, Json(req)).into_response()
        }
        None => (StatusCode::NOT_FOUND, format!("unknown cert")).into_response(),
    }
}
