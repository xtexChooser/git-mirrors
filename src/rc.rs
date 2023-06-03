use std::{fs, path::PathBuf};

use anyhow::{Context, Result};
use futures::try_join;
use podman_api::Podman;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::image::ImageResources;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash, Default)]
pub struct Resources {
    #[serde(default)]
    pub image: ImageResources,
}

impl Resources {
    pub fn load(path: &PathBuf) -> Result<Resources> {
        let mut res = Resources::default();
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            toml::from_str::<Resources>(&fs::read_to_string(path)?)
                .with_context(|| format!("load from {}", entry.path().display()))?
                .merge(&mut res);
        }
        Ok(res)
    }

    pub async fn apply(&self, api: &Podman) -> Result<()> {
        let images_api = api.images();
        try_join!(self.image.apply(&images_api))?;
        Ok(())
    }

    pub async fn purge(&self, api: &Podman) -> Result<()> {
        let images_api = api.images();
        try_join!(self.image.purge(&images_api))?;
        Ok(())
    }

    pub fn merge(self, new: &mut Self) {
        self.image.merge(&mut new.image);
    }
}

pub async fn apply(base: &PathBuf, podman: &Podman, keep: bool) -> Result<()> {
    let res = Resources::load(base)?;
    info!("configuration loaded");
    res.apply(&podman).await?;
    info!("resources applied");
    if !keep {
        res.purge(&podman).await?;
    }
    Ok(())
}
