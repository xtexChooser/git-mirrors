use std::time::Duration;

use anyhow::{Context, Result};
use duration_str::deserialize_duration;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::openssl::OpenSSLOpts;

#[derive(Debug, Serialize, Deserialize)]
pub struct ACLRule {
    pub certs: Vec<String>,
    #[serde(deserialize_with = "deserialize_duration")]
    pub max_expire: Duration,
    /// regex filters for SAN DNS names
    /// keep None if SAN copying and CN checking should be disabled
    /// empty list for matching everything
    #[serde(default)]
    pub allowed_san_dns: Option<Vec<String>>,
    #[serde(default)]
    pub can_custom_serial: bool,
    #[serde(default)]
    pub openssl_opt: OpenSSLOpts,
    #[serde(default)]
    pub can_custom_openssl_opts: bool,
    #[serde(default)]
    pub prefer_hash: Option<String>,
}

impl ACLRule {
    pub fn san_dns_to_regexs(&self) -> Result<Option<Vec<Regex>>> {
        if let Some(filters) = &self.allowed_san_dns {
            let mut regexs = Vec::new();
            for v in filters.iter() {
                regexs.push(Regex::new(v).context(format!("SAN DNS regex matcher {v}"))?);
            }
            Ok(Some(regexs))
        } else {
            Ok(None)
        }
    }
}
