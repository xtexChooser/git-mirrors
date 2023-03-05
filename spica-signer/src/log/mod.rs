use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{config::get_config, csr::CertReq};

use self::ntfy::NtfyLog;

pub mod ntfy;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogConfig {
    pub ntfy: Option<NtfyLog>,
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
        let config = &get_config().log;
        if let Some(log) = &config.ntfy {
            log.send(self).await?;
        }
        Ok(())
    }
}
