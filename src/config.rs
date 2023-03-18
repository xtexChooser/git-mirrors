use std::{env, fs, net::Ipv6Addr};

use anyhow::{Context, Error, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tracing::info;

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
    pub addr: AddrConfig,
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

fn default_tun_queues() -> usize {
    1
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AddrConfig {
    pub subnet: Ipv6Addr,
    #[serde(default = "default_addr_subnet_len")]
    pub subnet_len: u8,
    #[serde(default = "default_addr_index_len")]
    pub index_len: u8,
}

fn default_addr_subnet_len() -> u8 {
    64
}

fn default_addr_index_len() -> u8 {
    16
}

impl AddrConfig {
    pub fn host_addr(&self) -> Ipv6Addr {
        self.with_index(
            Ipv6Addr::from(u128::from(self.subnet) | (u128::MAX >> self.subnet_len)),
            0,
        )
    }

    pub fn with_index(&self, addr: Ipv6Addr, index: u16) -> Ipv6Addr {
        Ipv6Addr::from(u128::from(addr) >> self.index_len << self.index_len | index as u128)
    }
}
