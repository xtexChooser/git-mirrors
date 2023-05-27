use std::{env, fs, path::PathBuf};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash)]
pub struct Secrets {
    pub wmf: Option<Wmf>,
    pub dc: Dc,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash)]
pub struct Wmf {
    pub user: String,
    pub passwd: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash)]
pub struct Dc {
    pub url: String,
}

impl Secrets {
    pub fn new() -> Result<Secrets> {
        let path = Self::find_file()?;
        info!(
            path = format_args!("{}", path.display()),
            "located secrets.json"
        );
        let str = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&str)?)
    }

    fn find_file() -> Result<PathBuf> {
        if let Ok(path) = env::var("LYDIA_SECRETS_FILE") {
            let path = PathBuf::from(path);
            assert!(path.exists());
            return Ok(path);
        }
        let path = PathBuf::from("secrets.json");
        if path.exists() {
            return Ok(path);
        }
        let path = PathBuf::from("../secrets.json");
        if path.exists() {
            return Ok(path);
        }
        bail!("secrets.json not found")
    }
}
