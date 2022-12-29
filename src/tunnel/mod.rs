use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::peer::PeerConfig;

use self::wireguard::WireGuardConfig;

pub mod wireguard;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(tag = "type")]
pub enum TunnelConfig {
    WireGuard(WireGuardConfig),
}

impl TunnelConfig {
    pub fn link(self: &mut Self, peer: *const PeerConfig) {
        match self {
            Self::WireGuard(config) => config.link(peer),
        }
    }

    pub async fn create(peer: &PeerConfig) -> Result<()> {
        match &peer.tun {
            Self::WireGuard(config) => config.create(peer).await,
        }
    }

    pub async fn delete(peer: &PeerConfig) -> Result<()> {
        match &peer.tun {
            Self::WireGuard(config) => config.delete(peer).await,
        }
    }

    pub async fn update(peer: &PeerConfig) -> Result<()> {
        match &peer.tun {
            Self::WireGuard(config) => config.update(peer).await,
        }
    }

    pub async fn delete_unknown_if() -> Result<()> {
        WireGuardConfig::delete_unknown_if().await?;
        Ok(())
    }
}
