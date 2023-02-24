use serde::Deserialize;

use crate::openssl::OpenSSLOpts;

#[derive(Debug, Deserialize)]
pub struct CertConfig {
    pub name: String,
    pub file: String,
    #[serde(rename = "openssl-opt")]
    pub openssl_opt: OpenSSLOpts,
}
