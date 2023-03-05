use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::CertLog;

#[derive(Debug, Serialize, Deserialize)]
pub struct NtfyLog {
    pub url: String,
    #[serde(default)]
    pub token: Option<String>,
}

impl NtfyLog {
    pub async fn send(&self, log: &CertLog<'_>) -> Result<()> {
        let client = Client::new();
        let title = format!("{} / {}", log.ca, log.role);
        let body = log.cert.to_string();
        let mut req = client
            .post(&self.url)
            .body(body)
            .header("X-Title", title)
            .header("X-Tags", "spica-signer-log");
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }
        req.send().await?.error_for_status()?;
        Ok(())
    }
}
