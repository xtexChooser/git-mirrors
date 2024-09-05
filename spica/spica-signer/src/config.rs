use std::{env, fs};

use anyhow::{Context, Error, Result};
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing::info;

use crate::{cert::CertConfig, log::LogConfig, role::Role};

lazy_static! {
    static ref CONFIG: Config = load_config().unwrap();
}

pub fn load_config() -> Result<Config> {
    let path =
        env::var("SPICA_SIGNER_CONFIG").unwrap_or("/etc/spica/signer/spica-signer.toml".to_owned());
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
    pub listen_addr: String,
    #[serde(default = "default_otp_required")]
    pub otp_required: bool,
    #[serde(default = "default_show_roles")]
    pub show_roles: bool,
    pub cert: Vec<CertConfig>,
    pub role: Vec<Role>,
    #[serde(default)]
    pub log: Option<LogConfig>,
}

fn default_otp_required() -> bool {
    true
}

fn default_show_roles() -> bool {
    true
}
