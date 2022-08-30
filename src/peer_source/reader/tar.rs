use std::{fmt::Display, fs::File, io::read_to_string};

use anyhow::{Context, Result};
use inotify::WatchMask;
use tar::Archive;

use crate::{peer_conf::PeerConfig, peer_source::PeerSource};

use super::{file_watcher::FILE_WATCHER, Reader};

#[derive(Debug)]
pub struct TarReader {
    config: &'static PeerSource,
}

impl TarReader {
    pub fn new(config: &'static PeerSource) -> TarReader {
        TarReader { config }
    }
}

impl Reader for TarReader {
    fn get_config(&self) -> &'static PeerSource {
        self.config
    }

    fn collect(&self) -> Result<Vec<PeerConfig>> {
        Archive::new(File::open(&self.config.file).context("open tar file")?)
            .entries()
            .context("list files in tar")?
            .try_fold(Vec::new(), |mut peers, peer_file| {
                peers.push(PeerConfig::new(
                    read_to_string(peer_file.context("resolve tar entry")?)
                        .context("read tar entry to str")?,
                )?);
                Ok(peers)
            })
    }

    fn start_watching(&mut self) -> Result<()> {
        FILE_WATCHER
            .lock()?
            .add_watch(&self.config.file, WatchMask::CLOSE_WRITE)?;
        Ok(())
    }
}

impl Display for TarReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tar({:?})", self.config)
    }
}
