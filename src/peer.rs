use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{tunnel::TunnelConfig, zone::Zone};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PeerConfig {
    #[serde(skip)]
    pub zone: Option<&'static Zone>,
    pub tun: TunnelConfig,
}

impl PeerConfig {
    pub async fn create(self: &Self) -> Result<()> {
        Ok(())
    }

    pub async fn delete(self: &Self) -> Result<()> {
        Ok(())
    }

    pub async fn update(self: &Self) -> Result<()> {
        Ok(())
    }
}
