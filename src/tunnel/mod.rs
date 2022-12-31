use anyhow::{anyhow, bail, Result};

use crate::peer::PeerInfo;

use self::wireguard::WireGuardConfig;

pub mod wireguard;

pub const KEY_TUN_TYPE: &str = "tun_type";

pub const TUN_TYPE_NONE: &str = "none";
pub const TUN_TYPE_WIREGUARD: &str = "wireguard";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TunnelConfig {
    None,
    WireGuard(WireGuardConfig),
}

impl TunnelConfig {
    pub async fn new(peer: &PeerInfo) -> Result<TunnelConfig> {
        let kind = peer
            .props
            .get(KEY_TUN_TYPE)
            .ok_or(anyhow!("tun_type not found"))?;
        let conf = match kind.as_str() {
            TUN_TYPE_NONE => Self::None,
            TUN_TYPE_WIREGUARD => Self::WireGuard(WireGuardConfig::new(peer).await?),
            _ => bail!("unknown tun_type: {}", kind),
        };
        Ok(conf)
    }

    pub async fn create(&self, peer: &PeerInfo) -> Result<()> {
        match self {
            Self::None => Ok(()),
            Self::WireGuard(v) => v.update(peer).await,
        }
    }

    pub async fn update(&self, peer: &PeerInfo) -> Result<()> {
        match self {
            Self::None => Ok(()),
            Self::WireGuard(v) => v.update(peer).await,
        }
    }

    pub async fn del(&self, peer: &PeerInfo) -> Result<()> {
        match self {
            Self::None => Ok(()),
            Self::WireGuard(v) => v.del(peer).await,
        }
    }
}
