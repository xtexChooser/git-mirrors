use std::sync::Arc;

use anyhow::Result;
use reqwest::get;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::{error, info};

use crate::{
    chain::{self, Chain},
    config::get_config,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ResolverConfig {
    pub path: String,
    pub format_width: u16,
}

pub trait Resolver {
    async fn resolve(&self, index: u128) -> Result<Option<Chain>>;
}

impl ResolverConfig {
    pub fn to_full_path(&self, index: u128) -> String {
        format!(
            "{}{:0width$x}",
            self.path,
            index,
            width = self.format_width as usize
        )
    }
}

impl Resolver for ResolverConfig {
    async fn resolve(&self, index: u128) -> Result<Option<Chain>> {
        let path = self.to_full_path(index);
        let data = if path.starts_with("http") {
            let resp = get(path).await?;
            if (400..=499).contains(&resp.status().as_u16()) {
                return Ok(None);
            }
            resp.error_for_status()?.text().await?
        } else {
            if fs::try_exists(&path).await.map_err(anyhow::Error::from)? {
                return Ok(None);
            }
            fs::read_to_string(path)
                .await
                .map_err(anyhow::Error::from)?
        };
        Ok(Some(chain::parse_chain(&data)?))
    }
}

pub mod cache {
    use std::{
        collections::BTreeMap,
        sync::Arc,
        time::{Duration, Instant},
    };

    use lazy_static::lazy_static;
    use tokio::sync::Mutex;
    use tracing::info;

    use crate::chain::Chain;

    lazy_static! {
        pub(crate) static ref CACHE: Mutex<BTreeMap<u128, (Instant, Option<Arc<Chain>>)>> =
            Mutex::new(BTreeMap::new());
    }

    pub async fn find_cache(index: u128) -> Option<(Instant, Option<Arc<Chain>>)> {
        CACHE.lock().await.get(&index).cloned()
    }

    pub async fn put_cache(index: u128, chain: Option<Arc<Chain>>) {
        info!(index, resolved = chain.is_some(), "caching chain");
        CACHE.lock().await.insert(index, (Instant::now(), chain));
    }

    pub async fn purge() {
        let mut cache = CACHE.lock().await;
        let now = Instant::now();
        let mut outdated_indexes = vec![];
        for (index, (time, _)) in cache.iter() {
            if now.duration_since(*time) > Duration::from_secs(30) {
                outdated_indexes.push(*index);
            }
        }
        for index in outdated_indexes {
            cache.remove(&index);
        }
    }

    pub async fn gc_worker() {
        loop {
            purge().await;
            tokio::time::sleep(Duration::from_secs(15)).await;
        }
    }
}

pub async fn try_resolve(index: u128) -> Result<Option<Arc<Chain>>> {
    let _ = cache::CACHE.lock();
    if let Some((_, chain)) = cache::find_cache(index).await {
        return Ok(chain);
    }
    info!(index, "trying to resolve chain");
    for resolver in &get_config().resolver {
        if let Some(chain) = resolver.resolve(index).await? {
            let chain = Arc::new(chain);
            cache::put_cache(index, Some(chain.clone())).await;
            return Ok(Some(chain));
        }
    }
    cache::put_cache(index, None).await;
    Ok(None)
}

pub async fn resolve(index: u128) -> Option<Arc<Chain>> {
    match try_resolve(index).await {
        Ok(result) => result,
        Err(err) => {
            error!(err = err.to_string(), index, "failed to resolve chain");
            None
        }
    }
}
