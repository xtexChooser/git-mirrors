use anyhow::{Ok, Result};

use serde::Deserialize;

use crate::{config::get_config, peer::PeerConfig};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Zone {
    pub name: String,
    pub etcd_prefix: String,
    pub wireguard: Option<WireGuardConfig>,
    #[serde(skip)]
    pub peers: Vec<PeerConfig>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    pub ifname_prefix: String,
}

pub async fn init_zones() -> Result<()> {
    let zones = &get_config().await?.zone;
    for config in zones {
        init_zone(config).await?;
    }
    Ok(())
}

pub async fn init_zone(zone: &Zone) -> Result<()> {
    info!("initializing zone {}", zone.name);
    //zone.sync_peers();
    Ok(())
}
