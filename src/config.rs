use std::{cell::OnceCell, fs::read_to_string, path::PathBuf, sync::Mutex};

use anyhow::{bail, Result};
use serde::Deserialize;

use crate::args::ARGS;

pub static CONFIG: Mutex<OnceCell<Config>> = Mutex::new(OnceCell::new());

pub fn init_config() -> Result<()> {
    CONFIG.lock().unwrap().set(load_config()?).unwrap();
    Ok(())
}

pub fn load_config() -> Result<Config> {
    let path = locate_config()?;
    info!("loading configuration from {}", path.display());
    Ok(toml::from_str(read_to_string(path).unwrap().as_str())?)
}

pub fn locate_config() -> Result<PathBuf> {
    let path = ARGS
        .lock()
        .unwrap()
        .config
        .clone()
        .unwrap_or_else(|| PathBuf::from("peerd.toml"));
    if !path.exists() {
        bail!("configuration file not found at {}", path.display())
    }
    Ok(path)
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Config {
    pub etcd: EtcdConfig,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EtcdConfig {
    pub endpoints: Vec<String>,
    pub auth: Option<EtcdAuthConfig>,
    pub tls: Option<EtcdTlsConfig>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EtcdAuthConfig {
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct EtcdTlsConfig {
    pub domain: Option<String>,
    pub ca_cert_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
}
