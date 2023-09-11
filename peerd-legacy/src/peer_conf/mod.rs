use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use may::sync::Mutex;
use serde::Deserialize;

use crate::zone::reader::READERS;

use self::tunnel::PeerTunnelConfig;

pub mod tunnel;

lazy_static! {
    pub static ref PEERS: Mutex<HashMap<String, PeerConfig>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PeerConfig {
    pub id: String,
    #[serde(default)]
    pub tunnel: PeerTunnelConfig,
}

impl PeerConfig {
    pub fn from(file: String) -> Result<PeerConfig> {
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
                        let tun = crate::tunnel::Tunnel {
                            name: "test".to_string(),
                            zone: r.get_zone(),
                            config: Box::new(c.tunnel.wireguard.first().unwrap().to_owned()),
                        };
                        tun.get_manager().unwrap().add(&tun).unwrap();
                        peers.insert(c.id.clone(), c);
                    });
                Ok(())
            })
            .context("load peers")?;
        println!("Loaded {} peers", peers.len());
        Ok(())
    }
}
