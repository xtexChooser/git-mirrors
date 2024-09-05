use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{config::get_config, csr::CertReq};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub url: String,
    #[serde(default)]
    pub token: Option<String>,
}

pub struct CertLog<'a> {
    pub role: &'a str,
    pub ca: &'a str,
    pub req: &'a CertReq,
    pub log: &'a str,
    pub cert: &'a str,
}

impl<'a> CertLog<'a> {
    pub async fn send(&self) -> Result<()> {
        if let Some(config) = &get_config().log {
            let client = Client::new();

            let mut req = client
                .post(&config.url)
                .body(self.cert.to_string())
                .header("X-Title", format!("{} / {}", self.ca, self.role))
                .header("X-Tags", "spica-signer-log")
                .header("X-Role", self.role)
                .header("X-CA", self.ca)
                .header("X-CSR", serde_json::to_string(self.req)?)
                .header(
                    "X-Log",
                    base64::prelude::BASE64_STANDARD_NO_PAD.encode(self.log),
                );
            if let Some(token) = &config.token {
                req = req.bearer_auth(token);
            }

            req.send().await?.error_for_status()?;
        }
        Ok(())
    }
}
