use std::{fmt::Display, fs::File, io::read_to_string};

use anyhow::{Context, Ok, Result};
use inotify::WatchMask;

use crate::{peer_conf::PeerConfig, zone::Zone};

use super::{file_watcher::FILE_WATCHER, Reader};

#[derive(Debug)]
pub struct DirReader {
    zone: &'static Zone,
}

impl DirReader {
    pub fn new(zone: &'static Zone) -> DirReader {
        DirReader { zone }
    }
}

impl Reader for DirReader {
    fn get_zone(&self) -> &'static Zone {
        self.zone
    }

    fn collect(&self) -> Result<Vec<PeerConfig>> {
        let mut read_dir = self
            .zone
            .path
            .read_dir()
            .context("read dir for peer configs")?;
        read_dir.try_fold(Vec::new(), |mut peers, peer_file| {
            peers.push(PeerConfig::from(
                read_to_string(
                    File::open(peer_file.context("resolve dir entry path")?.path())
                        .context("open peer config file in dir")?,
                )
                .context("read peer config in dir to str")?,
            )?);
            Ok(peers)
        })
    }

    fn start_watching(&mut self) -> Result<()> {
        FILE_WATCHER
            .lock()?
            .add_watch(&self.zone.path, WatchMask::CLOSE_WRITE | WatchMask::DELETE)?;
        Ok(())
    }
}

impl Display for DirReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dir({:?})", self.zone)
    }
}
