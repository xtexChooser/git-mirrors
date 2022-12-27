use std::{cell::LazyCell, fs::read_to_string, path::PathBuf, sync::Mutex};

use anyhow::{bail, Result};
use serde::Deserialize;

use crate::args::ARGS;

pub static CONFIG: Mutex<LazyCell<Config>> = Mutex::new(LazyCell::new(|| {
    let path = locate_config().unwrap();
    info!("loading configuration from {}", path.display());
    toml::from_str(read_to_string(path).unwrap().as_str()).unwrap()
}));

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Config {
    /*    pub zone: Vec<Zone>,
    pub tunnel: TunnelConfig,
    pub routing: RoutingConfig,*/
}

pub fn locate_config() -> Result<PathBuf> {
    let mut path = ARGS
        .lock()
        .unwrap()
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from("peerd.toml"));
    if !path.exists() {
        path = PathBuf::from("/etc/peerd.toml");
    }
    if !path.exists() {
        bail!("configuration file not found")
    }
    Ok(path)
}
