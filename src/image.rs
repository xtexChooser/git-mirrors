use anyhow::{bail, Result};
use futures::TryStreamExt;
use podman_api::{
    api::Images,
    opts::{ImageListOpts, PullOpts, PullOptsBuilder, PullPolicy},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::direct_into_build;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash, Default)]
pub struct ImageResources {
    #[serde(default)]
    pub pulled: Vec<ImagePulled>,
    #[serde(default)]
    pub removed: Vec<ImageRemoved>,
}

impl ImageResources {
    pub async fn apply(&self, api: &Images) -> Result<()> {
        let list = api.list(&ImageListOpts::default()).await?;
        'pulled: for pulled in &self.pulled {
            if matches!(
                pulled.policy.to_owned().unwrap_or_default().as_str(),
                "" | "missing"
            ) {
                for image in &list {
                    if let Some(value) = &image.id {
                        if value == &pulled.name {
                            continue 'pulled;
                        }
                    }
                    if let Some(value) = &image.names {
                        for name in value {
                            if name == &pulled.name {
                                continue 'pulled;
                            }
                        }
                    }
                }
            }
            let mut report = api.pull(&pulled.clone().into());
            let mut error = String::new();
            while let Some(report) = report.try_next().await? {
                if let Some(images) = report.images {
                    for image in images {
                        info!(image, "pulled image")
                    }
                }
                if let Some(err) = report.error {
                    error.push_str(&err);
                }
                if let Some(stream) = report.stream {
                    info!(image = pulled.name, stream, "pulling image");
                }
            }
            if !error.is_empty() {
                bail!("error when pulling {}: {}", pulled.name, error)
            }
        }
        for removed in &self.removed {
            let remote = api.get(&removed.name);
            if remote.exists().await? {
                let force = removed.force.unwrap_or(false);
                if force {
                    remote.remove().await?;
                } else {
                    remote.delete().await?;
                }
                info!(name = removed.name, force, "deleted image");
            }
        }
        Ok(())
    }

    pub async fn purge(&self, api: &Images) -> Result<()> {
        let image = api.list(&ImageListOpts::default()).await?;
        let managed = self
            .pulled
            .iter()
            .map(|f| f.name.to_owned())
            .collect::<Vec<_>>();
        for image in image {
            if image.dangling.unwrap() || image.containers.unwrap_or(1) == 0 {
                let id = image.id.unwrap();
                if !managed.contains(&id)
                    && !image
                        .names
                        .unwrap_or_default()
                        .iter()
                        .any(|i| managed.contains(i))
                {
                    api.get(&id).delete().await?;
                    info!(name = id, "purged image");
                }
            }
        }
        Ok(())
    }

    pub fn merge(self, new: &mut Self) {
        new.pulled.extend(self.pulled);
        new.removed.extend(self.removed);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash, Default)]
pub struct ImagePulled {
    pub name: String,
    #[serde(default)]
    pub all_tags: Option<bool>,
    #[serde(default)]
    pub arch: Option<String>,
    #[serde(default)]
    pub os: Option<String>,
    #[serde(default)]
    pub policy: Option<String>,
    #[serde(default)]
    pub tls_verify: Option<bool>,
    #[serde(default)]
    pub variant: Option<String>,
}

impl Into<PullOptsBuilder> for ImagePulled {
    fn into(self) -> PullOptsBuilder {
        let mut builder = PullOpts::builder()
            .reference(self.name)
            .all_tags(self.all_tags.unwrap_or(false))
            .policy(
                self.policy
                    .map(|p| match p.as_str() {
                        "always" => PullPolicy::Always,
                        "missing" => PullPolicy::Missing,
                        "newer" => PullPolicy::Newer,
                        _ => PullPolicy::Missing,
                    })
                    .unwrap_or(PullPolicy::Missing),
            )
            .tls_verify(self.tls_verify.unwrap_or(true));
        if let Some(value) = self.arch {
            builder = builder.arch(value);
        }
        if let Some(value) = self.os {
            builder = builder.os(value);
        }
        if let Some(value) = self.variant {
            builder = builder.variant(value);
        }
        builder
    }
}

direct_into_build!(ImagePulled, PullOptsBuilder => PullOpts);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Clone, Hash, Default)]
pub struct ImageRemoved {
    pub name: String,
    #[serde(default)]
    pub force: Option<bool>,
}
