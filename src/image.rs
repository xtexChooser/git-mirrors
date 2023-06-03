use anyhow::{bail, Result};
use futures::TryStreamExt;
use podman_api::{
    api::Images,
    opts::{ImagesRemoveOpts, ImagesRemoveOptsBuilder, PullOpts, PullOptsBuilder, PullPolicy},
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
        for pulled in &self.pulled {
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
        Ok(())
    }

    pub async fn purge(&self, api: &Images) -> Result<()> {
        // todo
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

impl Into<ImagesRemoveOptsBuilder> for ImageRemoved {
    fn into(self) -> ImagesRemoveOptsBuilder {
        ImagesRemoveOpts::builder()
            .all(false)
            .ignore(true)
            .images(vec![self.name])
            .force(self.force.unwrap_or(false))
    }
}

direct_into_build!(ImageRemoved, ImagesRemoveOptsBuilder => ImagesRemoveOpts);
