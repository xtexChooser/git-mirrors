use std::{collections::HashMap, fs};

use anyhow::{bail, Context, Error, Result};
use lazy_static::lazy_static;
use openssl::x509::X509;
use serde::Deserialize;
use tracing::info;

use crate::{config::get_config, openssl::OpenSSLOpts};

#[derive(Debug, Deserialize)]
pub struct CertConfig {
    pub id: String,
    pub file: String,
    #[serde(rename = "openssl-opt", default)]
    pub openssl_opt: OpenSSLOpts,
}

impl CertConfig {
    pub fn read_file(&self) -> Result<String> {
        Ok(fs::read_to_string(self.file.to_owned()).map_err(Error::from)?)
    }
}

#[derive(Debug)]
pub struct CACert {
    pub config: &'static CertConfig,
    pub cert_pem: String,
    pub priv_key_pem: String,
    pub text: String,
}

impl CACert {
    pub fn new(config: &'static CertConfig) -> Result<CACert> {
        let full_pem = config.read_file()?;
        let cert_pem = Self::extract_pem(&full_pem, "CERTIFICATE")?;
        let priv_key_pem = Self::extract_pem(&full_pem, "PRIVATE KEY")?;
        let text = match Self::read_cert_text(&cert_pem) {
            Ok(text) => text,
            Err(err) => format!(
                "Error construting text dump for this certificate:\n{}",
                err.to_string()
            ),
        };
        Ok(CACert {
            config,
            cert_pem,
            priv_key_pem,
            text,
        })
    }

    fn extract_pem(pem: &String, tag: &str) -> Result<String> {
        for pem in pem::parse_many(pem)?.into_iter() {
            if pem.tag == tag {
                return Ok(pem::encode(&pem));
            }
        }
        bail!("pem with tag {} not found", tag)
    }

    pub fn read_cert_text(pem: &String) -> Result<String> {
        let x509 = X509::from_pem(pem.as_bytes())?;
        Ok(String::from_utf8(x509.to_text()?)?)
    }
}

lazy_static! {
    static ref CERTS: HashMap<String, CACert> = init_certs();
}

fn init_certs() -> HashMap<String, CACert> {
    let mut certs = HashMap::<String, CACert>::new();
    for config in get_config().cert.iter() {
        info!(id = config.id, "loading cert");
        let cert = CACert::new(config)
            .context(format!("Certificate config {}", config.id))
            .unwrap();
        if let Some(_) = certs.insert(config.id.to_owned(), cert) {
            panic!("duplicated cert name {}", config.id)
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
