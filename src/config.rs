use std::{env, fs};

use anyhow::{Context, Error, Result};
use lazy_static::lazy_static;
use serde::Deserialize;
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

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tun: TunConfig,
}

#[derive(Debug, Deserialize)]
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
