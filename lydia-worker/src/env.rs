use std::{fmt::Display, path::Path};

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

/// environment type
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum Env {
    /// the exozyme deployment
    Exo,
    /// the toolforge deployment
    TF,
    /// development environment
    Dev,
}

impl Display for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))?;
        Ok(())
    }
}

impl Env {
    pub fn is_exo(&self) -> bool {
        self == &Self::Exo
    }
    pub fn is_tf(&self) -> bool {
        self == &Self::TF
    }
    pub fn is_dev(&self) -> bool {
        self == &Self::Dev
    }
}

/// detect the current env type
pub fn detect_env() -> Result<Env> {
    let hostname = hostname::get()?
        .into_string()
        .map_err(|_| anyhow!("hostname is not valid utf-8"))?;
    let env = if hostname.contains("wikimedia.cloud") || hostname.starts_with("lydia-") {
        Env::TF
    } else if hostname == "exozyme" {
        Env::Exo
    } else if Path::new("../target/debug/lydia-worker").exists()
        || Path::new("./target/debug/lydia-worker").exists()
    {
        Env::Dev
    } else {
        bail!("unknown env with hostname {}", hostname)
    };
    info!(
        hostname,
        env = env.to_string(),
        "detected runtime environement"
    );
    Ok(env)
}
