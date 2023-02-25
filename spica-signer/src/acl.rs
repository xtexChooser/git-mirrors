use std::time::Duration;

use duration_str::deserialize_duration;
use serde::{Deserialize, Serialize};

use crate::openssl::OpenSSLOpts;

#[derive(Debug, Serialize, Deserialize)]
pub struct ACLRule {
    pub certs: Vec<String>,
    #[serde(deserialize_with = "deserialize_duration")]
    pub max_expire: Duration,
    #[serde(default)]
    pub allowed_san_dns: Vec<String>,
    #[serde(default)]
    pub can_custom_serial: bool,
    #[serde(default)]
    pub openssl_opt: OpenSSLOpts,
    #[serde(default)]
    pub prefer_hash: Option<String>,
}
