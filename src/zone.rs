use std::{
    collections::BTreeMap,
    sync::{Mutex, MutexGuard},
};

use anyhow::{anyhow, Ok, Result};
use serde::Deserialize;

use crate::config::get_config;

pub static mut ZONES: Mutex<BTreeMap<String, Zone>> = Mutex::new(BTreeMap::new());

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ZoneConfig {
    pub name: String,
    pub etcd_prefix: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Zone {
    pub config: ZoneConfig,
}

impl Zone {
    pub fn new(config: ZoneConfig) -> Zone {
        Zone { config }
    }
}

pub fn get_zones() -> Result<MutexGuard<'static, BTreeMap<String, Zone>>> {
    unsafe { ZONES.lock() }.map_err(|e| anyhow!("failed to lock ZONES {:?}", e))
}

pub async fn init_zones() -> Result<()> {
    let zones = &get_config()?.zone;
    for config in zones {
        init_zone(config.clone()).await?;
    }
    Ok(())
}

pub async fn init_zone(config: ZoneConfig) -> Result<()> {
    info!("initializing zone {}", config.name);
    let name = config.name.clone();
    let zone = Zone::new(config);
    //zone.sync_peers();
    get_zones()?.insert(name, zone);
    Ok(())
}
