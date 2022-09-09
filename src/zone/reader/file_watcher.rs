use std::thread::{self, JoinHandle};

use anyhow::Result;
use inotify::Inotify;
use lazy_static::lazy_static;
use may::sync::Mutex;

use crate::peer_conf::PeerConfig;

use super::READERS;

lazy_static! {
    pub static ref FILE_WATCHER: Mutex<Inotify> = Mutex::new(Inotify::init().unwrap());
}

pub fn start_watching_all() -> Result<()> {
    println!("Starting file watchers");
    READERS
        .lock()?
        .iter_mut()
        .filter(|r| r.get_zone().watch)
        .try_for_each(|r| r.start_watching())
}

pub fn start_watcher() -> JoinHandle<()> {
    thread::spawn(|| {
        let mut buffer = [0; 1024];
        loop {
            let modified = FILE_WATCHER
                .lock()
                .unwrap()
                .read_events_blocking(&mut buffer)
                .unwrap()
                .next()
                .is_some();
            if modified {
                println!("Peer source modification detected");
                let result = PeerConfig::reload();
                if result.is_err() {
                    println!("Error reloading configs: {:?}", result);
                }
            }
        }
    })
}
