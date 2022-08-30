use std::{fmt::Display, fs::File, io::read_to_string};

use anyhow::{Context, Ok, Result};
use inotify::WatchMask;

use crate::{peer_conf::PeerConfig, peer_source::PeerSource};

use super::{file_watcher::FILE_WATCHER, Reader};

#[derive(Debug)]
pub struct DirReader {
    config: &'static PeerSource,
}

impl DirReader {
    pub fn new(config: &'static PeerSource) -> DirReader {
        DirReader { config }
    }
}

impl Reader for DirReader {
    fn get_config(&self) -> &'static PeerSource {
        self.config
    }

    fn collect(&self) -> Result<Vec<PeerConfig>> {
        let mut read_dir = self
            .config
            .file
            .read_dir()
            .context("read dir for peer configs")?;
        read_dir.try_fold(Vec::new(), |mut peers, peer_file| {
            peers.push(PeerConfig::new(
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
        FILE_WATCHER.lock()?.add_watch(
            &self.config.file,
            WatchMask::CLOSE_WRITE | WatchMask::DELETE,
        )?;
        Ok(())
    }
}

impl Display for DirReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dir({:?})", self.config)
    }
}
