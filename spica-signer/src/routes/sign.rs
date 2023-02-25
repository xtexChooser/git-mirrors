use std::{collections::HashMap, time::Duration};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::post, Router};
use reqwest::header::CONTENT_TYPE;

use crate::{acl::ACLRule, cert::get_cert, csr::CertReq};

pub async fn make_router() -> Router {
    Router::new().route("/:id/sign/csr", post(sign_csr))
}

async fn sign_csr(Path(id): Path<String>, csr: String) -> impl IntoResponse {
    match get_cert(&id) {
        Some(ca_cert) => {
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
                prefer_hash: None,
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
            let crt = req.sign(ca_cert);
            let crt = match crt {
                Ok(crt) => crt,
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!(
                            "failed to sign cert: {}\n\ncert req: {:?}",
                            e.to_string(),
                            serde_json::to_string(&req)
                        ),
                    )
                        .into_response()
                }
            };
            (
                StatusCode::CREATED,
                [(CONTENT_TYPE, "application/x-x509-ca-cert")],
                crt,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, format!("unknown cert")).into_response(),
    }
}
