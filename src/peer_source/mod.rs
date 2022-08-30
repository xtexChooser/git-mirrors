use std::path::PathBuf;

use serde::Deserialize;

pub mod reader;

#[derive(Debug, Deserialize)]
pub struct PeerSource {
    pub file: PathBuf,
    #[serde(default = "default_watch")]
    pub watch: bool,
}

fn default_watch() -> bool {
    true
}
