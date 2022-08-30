use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use may::sync::Mutex;
use serde::Deserialize;

use crate::peer_source::reader::READERS;

lazy_static! {
    pub static ref PEERS: Mutex<Vec<PeerConfig>> = Mutex::new(Vec::new());
}

#[derive(Debug, Deserialize)]
pub struct PeerConfig {}

impl PeerConfig {
    pub fn new(file: String) -> Result<PeerConfig> {
        toml::from_str(file.as_str())
            .map_err(|e| anyhow!(format!("parse TOML peer conf: {}", e.to_string())))
    }

    pub fn reload() -> Result<()> {
        println!("Reloading peer configs");
        let mut peers = PEERS.lock()?;
        peers.clear();
        READERS
            .lock()?
            .iter()
            .try_for_each(|r| -> Result<()> {
                r.collect()
                    .context("collect peer configs")?
                    .into_iter()
                    .for_each(|c| {
                        peers.push(c);
                    });
                Ok(())
            })
            .context("load peers")?;
        println!("Loaded {} peers", peers.len());
        Ok(())
    }
}
