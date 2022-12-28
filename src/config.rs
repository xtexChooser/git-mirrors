use std::{
    cell::OnceCell,
    fs::read_to_string,
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

use anyhow::{anyhow, bail, Result};
use serde::Deserialize;

use crate::{args::get_args, etcd::EtcdConfig, zone::Zone};

pub static mut CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

pub fn init_config() -> Result<()> {
    unsafe { CONFIG.set(Mutex::new(load_config()?)) }
        .map_err(|e| anyhow!("config is already initialized {:?}", e))?;
    check_config()?;
    Ok(())
}

pub fn load_config() -> Result<Config> {
    let path = locate_config()?;
    info!("loading configuration from {}", path.display());
    Ok(toml::from_str(read_to_string(path).unwrap().as_str())?)
}

pub fn locate_config() -> Result<PathBuf> {
    let path = get_args()?
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from("peerd.toml"));
    if !path.exists() {
        bail!("configuration file not found at {}", path.display())
    }
    Ok(path)
}

pub fn get_config() -> Result<MutexGuard<'static, Config>> {
    Ok(unsafe { CONFIG.get() }
        .ok_or(anyhow!("config not initialized"))?
        .lock()
        .map_err(|e| anyhow!("failed to lock config {}", e))?)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Config {
    pub etcd: EtcdConfig,
    pub zone: Vec<Zone>,
    pub wireguard: WireGuardConfig,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    pub xplatform_exec: Option<String>,
    #[serde(default = "default_crc_if_peer_name")]
    pub crc_if_peer_name: bool,
}

fn default_crc_if_peer_name() -> bool {
    true
}

pub fn check_config() -> Result<()> {
    check_zone_name_conflict()?;
    check_zone_wg_prefix_conflict()?;
    Ok(())
}

pub fn check_zone_name_conflict() -> Result<()> {
    let config = get_config()?;
    for zone1 in &config.zone {
        for zone2 in &config.zone {
            if !std::ptr::eq(zone1, zone2) && zone1.name == zone2.name {
                bail!("two zones with the same name '{}' are defined", zone1.name);
            }
        }
    }
    Ok(())
}

pub fn check_zone_wg_prefix_conflict() -> Result<()> {
    let config = get_config()?;
    for zone1 in &config.zone {
        for zone2 in &config.zone {
            if let Some(wg1) = &zone1.wireguard && let Some(wg2) = &zone2.wireguard {
                if !std::ptr::eq(zone1, zone2) && (wg1.ifname_prefix.starts_with(&wg2.ifname_prefix) || wg2.ifname_prefix.starts_with(&wg1.ifname_prefix)) {
                    bail!("WG if name prefix in zone {} ({}), {} ({}) conflicted", zone1.name, wg1.ifname_prefix, zone2.name, wg2.ifname_prefix);
                }
            }
        }
    }
    Ok(())
}
