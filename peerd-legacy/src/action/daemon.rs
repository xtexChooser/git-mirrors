use crate::{
    peer_conf::PeerConfig,
    zone::reader::file_watcher::{start_watcher, start_watching_all},
};

pub fn daemon() {
    println!("Running peerd as daemon");
    PeerConfig::reload().unwrap();
    start_watching_all().unwrap();
    start_watcher().join().unwrap();
}
