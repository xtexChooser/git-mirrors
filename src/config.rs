use std::{fs::read_to_string, path::PathBuf};

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::peer_source::PeerSource;

lazy_static! {
    pub static ref CONFIG: Config = {
        let default_file = PathBuf::from("peerd.toml");
        let default_file_etc = PathBuf::from("/etc/peerd.toml");
        let mut file_path = crate::args::ARGS
            .config
            .as_ref()
            .unwrap_or_else(|| &default_file);
        if !file_path.exists() {
            file_path = &default_file_etc;
        }
        println!("Reading config from {}", file_path.display());
        return toml::from_str(read_to_string(file_path).unwrap().as_str()).unwrap();
    };
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub peer_source: Vec<PeerSource>,
}
