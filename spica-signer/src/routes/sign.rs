use anyhow::Result;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use axum_auth::AuthBasic;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use spica_signer_common::req::CSR;

use crate::{
    acl::ACLRule, cert::get_cert, config::get_config, csr::CertReq, log::CertLog,
    openssl::create_new_secp521r1_keypair, role::Role,
};

use super::auth::handle_auth;

pub async fn make_router() -> Router {
    Router::new()
        .route("/:id/sign/csr", post(sign_csr))
        .route("/:id/sign/json", post(sign_json))
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
    let req_pem = match pem::parse(&csr) {
        Ok(pem) => pem,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                format!("failed to parse csr pem: {e}"),
            )
                .into_response()
        }
    };
    sign(
        id,
        auth,
        params,
        |role, acl| CertReq::from_csr(&req_pem, &None, &None, acl, &role.prefer_hash),
        format!("origin CSR: {csr}\n").as_str(),
        "",
    )
    .await
    .into_response()
}

async fn sign_json(
    Path(id): Path<String>,
    auth: AuthBasic,
    Query(params): Query<SignParams>,
    Json(mut req): Json<CSR>,
) -> impl IntoResponse {
    let mut extra_log = format!("origin req: {req:#?}\n");
    let mut extra_crt = String::new();
    if req.public_key_pem.is_none() {
        match (|| {
            let key = create_new_secp521r1_keypair()?;
            let priv_key = String::from_utf8(key.private_key_to_pem()?)?;
            let pub_key = String::from_utf8(key.public_key_to_pem()?)?;
            Ok::<(String, String), anyhow::Error>((priv_key, pub_key))
        })() {
            Ok((priv_key, pub_key)) => {
                extra_log.push_str("auto generated secp521r1 keypair\n");
                extra_crt.push_str(&(priv_key + &pub_key));
                req.public_key_pem = Some(pub_key);
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("failed to auto gen secp521r1 keypair: {e}"),
                )
                    .into_response()
            }
        }
    }
    sign(
        id,
        auth,
        params,
        |role, acl| CertReq::from_json(&req, acl, &role.prefer_hash),
        &extra_log,
        &extra_crt,
    )
    .await
    .into_response()
}

async fn sign<F>(
    id: String,
    auth: AuthBasic,
    params: SignParams,
    f: F,
    extra_log: &str,
    extra_crt: &str,
) -> impl IntoResponse
where
    F: Fn(&Role, &ACLRule) -> Result<CertReq>,
{
    let role = match handle_auth(auth).await {
        Ok(role) => role,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };
    match get_cert(&id) {
        Some(ca_cert) => {
            let mut log = String::from(extra_log);
            let mut internal_err = false;
            for acl in role.acl.iter() {
                log.push_str("====================\n");
                if get_config().show_roles {
                    log.push_str(format!("ACL definition: {:#?}\n", &acl).as_str());
                }
                let req = f(role, acl);
                let req = match req {
                    Ok(req) => req,
                    Err(e) => {
                        log.push_str(format!("ACL rejected: {e}\n").as_str());
                        continue;
                    }
                };
                log.push_str(format!("created req: {:#?}\n", &req).as_str());
                let crt = match req.sign(ca_cert) {
                    Ok(crt) => crt,
                    Err(e) => {
                        log.push_str(format!("sign failed: {e}\n").as_str());
                        internal_err = true;
                        continue;
                    }
                };
                log.push_str("certificate signed\n");
                log.push_str("sending to cert log\n");
                let ct_log = CertLog {
                    role: &role.id,
                    ca: &ca_cert.config.id,
                    req: &req,
                    log: &log,
                    cert: &crt,
                };
                if let Err(e) = ct_log.send().await {
                    log.push_str(format!("ct log send failed: {e}\n").as_str());
                    internal_err = true;
                    continue;
                }
                log.push_str("certificate created\n");
                return (
                    StatusCode::CREATED,
                    [(CONTENT_TYPE, "application/x-x509-ca-cert")],
                    if params.log.unwrap_or(false) {
                        log + &crt + extra_crt
                    } else {
                        crt + extra_crt
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
        None => (StatusCode::NOT_FOUND, "unknown cert".to_string()).into_response(),
    }
}
