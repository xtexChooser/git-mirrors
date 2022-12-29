use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{tunnel::TunnelConfig, zone::Zone};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PeerConfig {
    #[serde(skip)]
    pub zone: Option<&'static Zone>,
    #[serde(skip)]
    pub name: Option<String>,
    pub tun: TunnelConfig,
}

impl PeerConfig {
    pub fn link(self: &mut Self, zone: &'static Zone, name: String) {
        self.zone = Some(zone);
        self.name = Some(name);
        self.tun.link(self);
    }

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
