use std::{cmp, collections::HashMap};

use anyhow::{Context, Result};
use podman_api::{
    api::Volumes,
    models::VolumeInspect,
    opts::{VolumeCreateOpts, VolumeCreateOptsBuilder},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::direct_into_build;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct VolumeResources {
    #[serde(default)]
    pub created: Vec<VolumeCreated>,
    #[serde(default)]
    pub removed: Vec<VolumeRemoved>,
}

impl VolumeResources {
    pub async fn apply(&self, api: &Volumes) -> Result<()> {
        for created in &self.created {
            let remote_vol = api.get(&created.name);
            if remote_vol.exists().await? {
                if created == &remote_vol.inspect().await? {
                    continue;
                } else {
                    remote_vol.delete().await?;
                    info!(
                        name = created.name,
                        "deleted exists volume for not matching"
                    )
                }
            }
            let resp = api
                .create(&created.clone().into())
                .await
                .with_context(|| format!("target volume: {}", &created.name))?;
            info!(
                name = created.name,
                response = serde_json::to_string(&resp)?,
                "created volume"
            );
        }
        for removed in &self.removed {
            let remote_vol = api.get(&removed.name);
            if remote_vol.exists().await? {
                let force = removed.force.unwrap_or(false);
                if force {
                    remote_vol.remove().await?;
                } else {
                    remote_vol.delete().await?;
                }
                info!(name = removed.name, force, "deleted exists volume");
            }
        }
        Ok(())
    }

    pub async fn purge(&self, api: &Volumes) -> Result<()> {
        // todo
        Ok(())
    }

    pub fn merge(self, new: &mut Self) {
        new.created.extend(self.created);
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct VolumeCreated {
    pub name: String,
    pub driver: String,
    #[serde(default)]
    pub options: HashMap<String, String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

impl Into<VolumeCreateOptsBuilder> for VolumeCreated {
    fn into(self) -> VolumeCreateOptsBuilder {
        let mut builder = VolumeCreateOpts::builder()
            .name(self.name)
            .driver(self.driver)
            .options(self.options)
            .labels(self.labels);
        builder
    }
}

impl cmp::PartialEq<VolumeInspect> for VolumeCreated {
    fn eq(&self, other: &VolumeInspect) -> bool {
        self.name == other.name
            && self.driver == other.driver
            && self.options == other.options
            && self.labels == other.labels
    }
}

direct_into_build!(VolumeCreated, VolumeCreateOptsBuilder => VolumeCreateOpts);

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Default)]
pub struct VolumeRemoved {
    pub name: String,
    #[serde(default)]
    pub force: Option<bool>,
}
