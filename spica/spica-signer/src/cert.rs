use std::{collections::HashMap, fs};

use anyhow::{bail, Context, Error, Result};
use lazy_static::lazy_static;
use openssl::{
    pkey::{PKey, Private},
    x509::X509,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{config::get_config, openssl::OpenSSLOpts};

#[derive(Debug, Serialize, Deserialize)]
pub struct CertConfig {
    pub id: String,
    pub file: String,
    #[serde(default)]
    pub openssl_opt: OpenSSLOpts,
    #[serde(default)]
    pub priv_key_pass: Option<String>,
}

impl CertConfig {
    pub fn read_file(&self) -> Result<String> {
        fs::read_to_string(&self.file).map_err(Error::from)
    }
}

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
                "Error construting text dump for this certificate:\n{err}"
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

    pub fn to_ossl_x509(&self) -> Result<X509> {
        Ok(X509::from_pem(self.cert_pem.as_bytes())?)
    }

    pub fn to_ossl_pkey(&self) -> Result<PKey<Private>> {
        match &self.config.priv_key_pass {
            Some(pwd) => Ok(PKey::private_key_from_pem_passphrase(
                self.priv_key_pem.as_bytes(),
                pwd.as_bytes(),
            )?),
            None => Ok(PKey::private_key_from_pem(self.priv_key_pem.as_bytes())?),
        }
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
        if certs.insert(config.id.to_owned(), cert).is_some() {
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
