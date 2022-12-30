use std::{
    collections::{BTreeMap, VecDeque},
    pin::Pin,
};

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
    let mut zones = get_config()
        .await?
        .zone
        .drain(..)
        .collect::<VecDeque<ZoneConfig>>();
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

    PeerConfig::new(
        Pin::new(zone),
        "test".to_string(),
        BTreeMap::from([
            ("tun_type".to_string(), "wireguard".to_string()),
            (
                "wg_private_key".to_string(),
                "QMTsMxn1RRMlUcEGp+yC6T8ue7fAyvUncy3RU8uha1c=".to_string(),
            ),
            (
                "wg_remote_public_key".to_string(),
                "8VItif1ki24i+k2wcx+FOeA0/DPB3qv5ST3xhubLJEw=".to_string(),
            ),
        ]),
    )
    .await?
    .create()
    .await?;
    Ok(())
}
