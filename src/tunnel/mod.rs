use anyhow::{anyhow, bail, Result};

use crate::peer::PeerInfo;

use self::wireguard::WireGuardConfig;

pub mod wireguard;

pub const KEY_TUN_TYPE: &str = "tun_type";

pub const TUN_TYPE_WIREGUARD: &str = "wireguard";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TunnelConfig {
    WireGuard(WireGuardConfig),
}

impl TunnelConfig {
    pub fn new(peer: &PeerInfo) -> Result<TunnelConfig> {
        let kind = peer
            .props
            .get(KEY_TUN_TYPE)
            .ok_or(anyhow!("tun_type not found"))?;
        let conf = match kind.as_str() {
            TUN_TYPE_WIREGUARD => Self::WireGuard(WireGuardConfig::new(peer)?),
            _ => bail!("unknown tun_type: {}", kind),
        };
        Ok(conf)
    }

    pub async fn create(self: &Self, peer: &PeerInfo) -> Result<()> {
        match self {
            Self::WireGuard(v) => v.update(peer),
        }
    }

    pub async fn update(self: &Self, peer: &PeerInfo) -> Result<()> {
        match self {
            Self::WireGuard(v) => v.update(peer),
        }
    }

    pub async fn del(self: &Self, peer: &PeerInfo) -> Result<()> {
        match self {
            Self::WireGuard(v) => v.del(peer),
        }
    }
}
