use std::path::PathBuf;

use serde::Deserialize;

pub mod reader;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct PeerSource {
    pub file: PathBuf,
    #[serde(default = "default_watch")]
    pub watch: bool,
}

fn default_watch() -> bool {
    true
}
