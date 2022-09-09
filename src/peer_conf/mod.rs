use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use may::sync::Mutex;
use serde::Deserialize;

use crate::zone::reader::READERS;

use self::tunnel::TunnelConfig;

pub mod tunnel;

lazy_static! {
    pub static ref PEERS: Mutex<HashMap<String, PeerConfig>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PeerConfig {
    pub id: String,
    #[serde(default)]
    pub tunnel: TunnelConfig,
}

impl PeerConfig {
    pub fn new(file: String) -> Result<PeerConfig> {
        toml::from_str(file.as_str())
            .map_err(|e| anyhow!(format!("parse TOML peer conf: {}", e.to_string())))
    }

    pub fn reload() -> Result<()> {
        println!("Reloading peer configs");
        let mut peers = PEERS.lock()?;
        let old_peers = peers.clone();
        peers.clear();
        READERS
            .lock()?
            .iter()
            .try_for_each(|r| -> Result<()> {
                r.collect()
                    .context("collect peer configs")?
                    .into_iter()
                    .for_each(|c| {
                        peers.insert(c.id.clone(), c);
                    });
                Ok(())
            })
            .context("load peers")?;
        println!("Loaded {} peers", peers.len());
        Ok(())
    }
}
