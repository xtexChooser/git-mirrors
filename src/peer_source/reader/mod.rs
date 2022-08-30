use std::fmt::Display;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use may::sync::Mutex;

use crate::{config::CONFIG, peer_conf::PeerConfig};

use self::{dir::DirReader, tar::TarReader};

use super::PeerSource;

pub mod dir;
pub mod file_watcher;
pub mod tar;

lazy_static! {
    pub static ref READERS: Mutex<Box<Vec<Box<(dyn Reader + Send + Sync)>>>> =
        Mutex::new(Box::new(
            CONFIG
                .peer_source
                .iter()
                .map(|s| <dyn Reader>::new(s).unwrap())
                .collect()
        ));
}

pub trait Reader: Display {
    fn get_config(&self) -> &'static PeerSource;
    fn collect(&self) -> Result<Vec<PeerConfig>>;
    fn start_watching(&mut self) -> Result<()>;
}

impl dyn Reader {
    fn new(source: &'static PeerSource) -> Result<Box<(dyn Reader + Send + Sync)>> {
        if !source.file.exists() {
            return Err(anyhow!("file not exists: {}", source.file.display()));
        } else if source.file.is_file() {
            return Ok(Box::new(TarReader::new(source)));
        } else {
            return Ok(Box::new(DirReader::new(source)));
        }
    }
}
