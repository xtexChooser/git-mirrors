use std::collections::HashMap;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing::info;

use crate::{config::get_config, openssl::OpenSSLOpts};

#[derive(Debug, Deserialize)]
pub struct CertConfig {
    pub name: String,
    pub file: String,
    #[serde(rename = "openssl-opt", default)]
    pub openssl_opt: OpenSSLOpts,
}

#[derive(Debug)]
pub struct CACert {
    pub config: &'static CertConfig,
}

impl CACert {
    pub fn new(config: &'static CertConfig) -> Result<CACert> {
        Ok(CACert { config })
    }
}

lazy_static! {
    static ref CERTS: HashMap<String, CACert> = init_certs();
}

fn init_certs() -> HashMap<String, CACert> {
    let mut certs = HashMap::<String, CACert>::new();
    for config in get_config().cert.iter() {
        info!(name = config.name, "loading cert");
        let cert = CACert::new(config)
            .context(format!("Certificate config {}", config.name))
            .unwrap();
        if let Some(_) = certs.insert(config.name.to_owned(), cert) {
            panic!("duplicated cert name {}", config.name)
        }
    }
    certs
}

pub fn get_certs() -> &'static HashMap<String, CACert> {
    &CERTS
}

pub fn get_cert(name: &String) -> Option<&'static CACert> {
    CERTS.get(name)
}
