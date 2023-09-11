use std::fmt::Display;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use may::sync::Mutex;

use crate::{config::CONFIG, peer_conf::PeerConfig};

use self::{dir::DirReader, tar::TarReader};

use super::Zone;

pub mod dir;
pub mod file_watcher;
pub mod tar;

lazy_static! {
    pub static ref READERS: Mutex<Box<Vec<Box<(dyn Reader + Send + Sync)>>>> =
        Mutex::new(Box::new(
            CONFIG
                .zone
                .iter()
                .map(|s| <dyn Reader>::new(s).unwrap())
                .collect()
        ));
}

pub trait Reader: Display {
    fn get_zone(&self) -> &'static Zone;
    fn collect(&self) -> Result<Vec<PeerConfig>>;
    fn start_watching(&mut self) -> Result<()>;
}

impl dyn Reader {
    fn new(zone: &'static Zone) -> Result<Box<(dyn Reader + Send + Sync)>> {
        if !zone.path.exists() {
            return Err(anyhow!("file not exists: {}", zone.path.display()));
        } else if zone.path.is_file() {
            return Ok(Box::new(TarReader::new(zone)));
        } else {
            return Ok(Box::new(DirReader::new(zone)));
        }
    }
}
