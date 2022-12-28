use std::{
    cell::OnceCell,
    fs::read_to_string,
    path::PathBuf,
    sync::{Mutex, MutexGuard},
};

use anyhow::{anyhow, bail, Result};
use serde::Deserialize;

use crate::{args::get_args, etcd::EtcdConfig, zone::ZoneConfig};

pub static mut CONFIG: OnceCell<Mutex<Config>> = OnceCell::new();

pub fn init_config() -> Result<()> {
    unsafe { CONFIG.set(Mutex::new(load_config()?)) }
        .map_err(|e| anyhow!("config is already initialized {:?}", e))?;
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
    pub zone: Vec<ZoneConfig>,
}
