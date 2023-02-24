use std::{collections::HashMap, env, fs};

use anyhow::{Context, Error, Result};
use lazy_static::lazy_static;
use serde::Deserialize;
use tracing::info;

use crate::cert::CertConfig;

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
    pub cert: HashMap<String, CertConfig>,
}
