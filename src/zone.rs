use std::{collections::BTreeMap, pin::Pin};

use anyhow::{Ok, Result};

use serde::Deserialize;

use crate::{config::get_config, peer::PeerConfig};

pub static mut ZONES: Vec<Zone> = Vec::new();

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ZoneConfig {
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Zone {
    pub conf: ZoneConfig,
}

pub async fn init_zones() -> Result<()> {
    let mut config = get_config().await?;
    let zones = &mut config.zone;
    while let Some(config) = zones.pop_front() {
        init_zone(config).await?;
    }
    Ok(())
}

pub async fn init_zone(conf: ZoneConfig) -> Result<()> {
    info!("initializing zone {}", conf.name);
    let zone = Zone { conf };
    unsafe {
        ZONES.push(zone);
    }
    let zone = unsafe { ZONES.last() }.unwrap();
    //zone.sync_peers();

    Ok(())
}
