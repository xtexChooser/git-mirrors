use std::{cell::OnceCell, fs::read_to_string, path::PathBuf};

use anyhow::{anyhow, bail, Result};
use serde::Deserialize;
use tokio::sync::{Mutex, MutexGuard};

use crate::{args::get_args, etcd::EtcdConfig, zone::ZoneConfig};

pub static mut CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

pub async fn init_config() -> Result<()> {
    unsafe { CONFIG.set(Mutex::new(load_config().await?)) }
        .map_err(|e| anyhow!("config is already initialized {:?}", e))?;
    check_config().await?;
    Ok(())
}

pub async fn load_config() -> Result<Config> {
    let path = locate_config().await?;
    info!("loading configuration from {}", path.display());
    Ok(toml::from_str(read_to_string(path).unwrap().as_str())?)
}

pub async fn locate_config() -> Result<PathBuf> {
    let path = get_args()
        .await?
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from("peerd.toml"));
    if !path.exists() {
        bail!("configuration file not found at {}", path.display())
    }
    Ok(path)
}

pub async fn get_config() -> Result<MutexGuard<'static, Config>> {
    Ok(unsafe { CONFIG.get() }
        .ok_or(anyhow!("config not initialized"))?
        .lock()
        .await)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Config {
    pub etcd: EtcdConfig,
    pub zone: Vec<ZoneConfig>,
    pub wireguard: Option<WireGuardConfig>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    pub xplatform_exec: Option<String>,
    #[serde(default = "default_crc_if_peer_name")]
    pub crc_if_peer_name: bool,
    #[serde(default = "default_prefer_ipv6")]
    pub prefer_ipv6: bool,
}

fn default_crc_if_peer_name() -> bool {
    true
}

fn default_prefer_ipv6() -> bool {
    true
}

pub async fn check_config() -> Result<()> {
    check_zone_name_conflict().await?;
    check_zone_wg_prefix_conflict().await?;
    Ok(())
}

pub async fn check_zone_name_conflict() -> Result<()> {
    let config = get_config().await?;
    for zone1 in &config.zone {
        for zone2 in &config.zone {
            if !std::ptr::eq(zone1, zone2) && zone1.name == zone2.name {
                bail!("two zones with the same name '{}' are defined", zone1.name);
            }
        }
    }
    Ok(())
}

pub async fn check_zone_wg_prefix_conflict() -> Result<()> {
    let config = get_config().await?;
    for zone1 in &config.zone {
        for zone2 in &config.zone {
            if let Some(wg1) = &zone1.wireguard && let Some(wg2) = &zone2.wireguard && !std::ptr::eq(zone1, zone2) && (wg1.ifname_prefix.starts_with(&wg2.ifname_prefix) || wg2.ifname_prefix.starts_with(&wg1.ifname_prefix)) {
                bail!("WG if name prefix in zone {} ({}), {} ({}) conflicted", zone1.name, wg1.ifname_prefix, zone2.name, wg2.ifname_prefix);
            }
        }
    }
    Ok(())
}
