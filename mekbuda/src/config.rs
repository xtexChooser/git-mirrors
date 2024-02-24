use std::{env, fs};

use anyhow::{Context, Error, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{dns::DnsConfig, resolver::ResolverConfig, subnet::SubnetConfig};

lazy_static! {
    static ref CONFIG: Config = load_config().unwrap();
}

pub fn load_config() -> Result<Config> {
    let path = env::var("MEKBUDA_CONFIG").unwrap_or("mekbuda.toml".to_owned());
    info!(path, "loading config from file");
    let text = fs::read_to_string(&path)
        .map_err(Error::from)
        .context(format!("load config from {}", &path))?;
    let config = toml::from_str::<Config>(&text)?;
    info!("loaded config");
    Ok(config)
}

pub fn get_config() -> &'static Config {
    &CONFIG
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct Config {
    pub tun: TunConfig,
    pub dns: DnsConfig,
    pub subnet: Vec<SubnetConfig>,
    pub resolver: Vec<ResolverConfig>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TunConfig {
    #[serde(default = "default_tun_ifname")]
    pub ifname: String,
    #[serde(default = "default_tun_queues")]
    pub queues: usize,
}

fn default_tun_ifname() -> String {
    String::from("mekbuda")
}

const fn default_tun_queues() -> usize {
    1
}
