use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use axum_auth::AuthBasic;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

use crate::{cert::get_cert, csr::CertReq};

use super::auth::handle_auth;

pub async fn make_router() -> Router {
    Router::new().route("/:id/sign/csr", post(sign_csr))
}

#[derive(Debug, Deserialize)]
struct SignParams {
    log: Option<bool>,
}

async fn sign_csr(
    Path(id): Path<String>,
    auth: AuthBasic,
    Query(params): Query<SignParams>,
    csr: String,
) -> impl IntoResponse {
    let role = match handle_auth(auth).await {
        Ok(role) => role,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };
    match get_cert(&id) {
        Some(ca_cert) => {
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
            let mut log = String::new();
            let mut internal_err = false;
            for acl in role.acl.iter() {
                log.push_str("====================\n");
                log.push_str(
                    format!("ACL definition: {:?}\n", serde_json::to_string(&acl)).as_str(),
                );
                let req = CertReq::from_csr(&req_pem, &None, &None, acl, &role.prefer_hash);
                let req = match req {
                    Ok(req) => req,
                    Err(e) => {
                        log.push_str(format!("ACL rejected: {}\n", e.to_string()).as_str());
                        continue;
                    }
                };
                log.push_str(format!("created req: {:?}\n", serde_json::to_string(&req)).as_str());
                let crt = match req.sign(ca_cert) {
                    Ok(crt) => crt,
                    Err(e) => {
                        log.push_str(format!("sign failed: {}\n", e.to_string()).as_str());
                        internal_err = true;
                        continue;
                    }
                };
                log.push_str("certificate signed\n");
                log.push_str("sending to cert log\n");
                // @TODO: send to log
                log.push_str("certificate created\n");
                return (
                    StatusCode::CREATED,
                    [(CONTENT_TYPE, "application/x-x509-ca-cert")],
                    if params.log.unwrap_or(false) {
                        log + &crt
                    } else {
                        crt
                    },
                )
                    .into_response();
            }
            (
                if internal_err {
                    StatusCode::INTERNAL_SERVER_ERROR
                } else {
                    StatusCode::FORBIDDEN
                },
                log,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, format!("unknown cert")).into_response(),
    }
}
