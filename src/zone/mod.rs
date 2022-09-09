use std::path::PathBuf;

use serde::Deserialize;

pub mod reader;

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Zone {
    pub path: PathBuf,
    #[serde(default = "default_watch")]
    pub watch: bool,
}

fn default_watch() -> bool {
    true
}
