use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use podman_api::Podman;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    constant::ENV_PURGE_VOLS, container::ContainerResources, image::ImageResources,
    network::NetworkResources, volume::VolumeResources,
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct Resources {
    #[serde(default)]
    pub image: ImageResources,
    #[serde(default)]
    pub volume: VolumeResources,
    #[serde(default)]
    pub network: NetworkResources,
    #[serde(default)]
    pub container: ContainerResources,
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
        self.image
            .apply(&api.images())
            .await
            .context("apply images")?;
        self.volume
            .apply(&api.volumes())
            .await
            .context("apply volumes")?;
        self.network
            .apply(&api.networks())
            .await
            .context("apply networks")?;
        self.container
            .apply(&api.containers())
            .await
            .context("apply containers")?;
        Ok(())
    }

    pub async fn purge(&self, api: &Podman) -> Result<()> {
        self.image
            .purge(&api.images())
            .await
            .context("purge images")?;
        if env::var(ENV_PURGE_VOLS).unwrap_or_default() == "true" {
            self.volume
                .purge(&api.volumes())
                .await
                .context("purge volumes")?;
        }
        self.network
            .purge(&api.networks())
            .await
            .context("purge networks")?;
        self.container
            .purge(&api.containers())
            .await
            .context("purge containers")?;
        Ok(())
    }

    pub fn merge(self, new: &mut Self) {
        self.image.merge(&mut new.image);
        self.volume.merge(&mut new.volume);
        self.network.merge(&mut new.network);
        self.container.merge(&mut new.container);
    }
}

pub async fn apply(base: &PathBuf, podman: &Podman, keep: bool) -> Result<()> {
    let res = Resources::load(base)?;
    info!("configuration loaded");
    res.apply(&podman).await.context("apply resources")?;
    info!("resources applied");
    if !keep {
        res.purge(&podman).await.context("purge resources")?;
    }
    Ok(())
}
