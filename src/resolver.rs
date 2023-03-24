use anyhow::Result;
use reqwest::get;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::chain::{self, Chain};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ResolverConfig {
    pub path: String,
    pub format_width: u16,
}

pub trait Resolver {
    async fn resolve(&self, id: u128) -> Result<Option<Chain>>;
}

impl ResolverConfig {
    pub fn to_full_path(&self, id: u128) -> String {
        format!(
            "{}{:0width$x}",
            self.path,
            id,
            width = self.format_width as usize
        )
    }
}

impl Resolver for ResolverConfig {
    async fn resolve(&self, id: u128) -> Result<Option<Chain>> {
        let path = self.to_full_path(id);
        let data = if path.starts_with("http") {
            let resp = get(path).await?;
            if (400..=499).contains(&resp.status().as_u16()) {
                return Ok(None);
            }
            resp.error_for_status()?.text().await?
        } else {
            if fs::try_exists(&path).await.map_err(anyhow::Error::from)? {
                return Ok(None);
            }
            fs::read_to_string(path)
                .await
                .map_err(anyhow::Error::from)?
        };
        Ok(Some(chain::parse_chain(&data)?))
    }
}
