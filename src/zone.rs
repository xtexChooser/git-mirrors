use std::{
    collections::{BTreeMap, VecDeque},
    slice::IterMut,
};

use anyhow::{anyhow, bail, Ok, Result};

use etcd_client::GetOptions;
use serde::Deserialize;
use serde_json::Value;

use crate::{config::get_config, etcd::get_etcd_client, peer::PeerConfig};

pub static mut ZONES: Vec<Zone> = Vec::new();

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ZoneConfig {
    pub name: String,
    pub etcd_prefix: String,
    pub wireguard: Option<WireGuardConfig>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    pub ifname_prefix: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Zone {
    pub conf: ZoneConfig,
    pub index: usize,
    pub peers: Vec<PeerConfig>,
}

pub async fn init_zones() -> Result<()> {
    let mut zones = get_config()
        .await?
        .zone
        .drain(..)
        .collect::<VecDeque<ZoneConfig>>();
    while let Some(config) = zones.pop_front() {
        init_zone(config).await?;
    }
    Ok(())
}

pub async fn init_zone(conf: ZoneConfig) -> Result<()> {
    info!("initializing zone {}", conf.name);
    let zone = Zone {
        conf,
        index: 0,
        peers: vec![],
    };
    unsafe {
        ZONES.push(zone);
    }
    let mut zone = unsafe { ZONES.last_mut() }.unwrap();
    zone.index = unsafe { ZONES.len() } - 1;
    zone.sync_all_peers().await?;
    Ok(())
}

pub fn get_zones() -> IterMut<'static, Zone> {
    unsafe { ZONES.iter_mut() }
}

impl Zone {
    pub fn get(index: usize) -> &'static Self {
        unsafe { ZONES.get_mut(index) }.unwrap()
    }

    pub fn get_peer_key(&self, name: &str) -> String {
        self.conf.etcd_prefix.to_owned() + name
    }

    pub async fn sync_all_peers(&mut self) -> Result<()> {
        let zone_name = self.conf.name.to_owned();
        let prefix = self.conf.etcd_prefix.to_owned();
        let kvs = get_etcd_client()
            .await?
            .get(
                prefix.as_str(),
                Some(GetOptions::new().with_prefix().with_keys_only()),
            )
            .await?
            .kvs()
            .to_owned();
        for kv in &kvs {
            let full_key = kv.key_str()?.to_owned();
            assert!(full_key.starts_with(&prefix));
            let mut key = full_key.clone();
            key.drain(0..prefix.len());
            info!("found exist peer {} in {}", key, zone_name);
            if let Err(err) = self.handle_peer(key.as_str()).await {
                error!(
                    "failed to import peer {} in {}, disabling: {}",
                    key, zone_name, err
                );
                let mut etcd = get_etcd_client().await?;
                let val = etcd
                    .get(full_key.as_str(), None)
                    .await?
                    .kvs()
                    .first()
                    .unwrap()
                    .value_str()?
                    .to_owned();
                etcd.delete(full_key.as_str(), None).await?;
                etcd.put(r".".to_string() + &full_key, val, None).await?;
            }
        }
        Ok(())
    }

    pub async fn handle_peer(&mut self, name: &str) -> Result<()> {
        if name.starts_with('_') {
            return Ok(());
        }

        let key = self.get_peer_key(name);
        if !is_valid_peer_name(name) {
            bail!("invalid peer name: {}", name)
        }

        let mut conf_kvs: BTreeMap<String, String> = BTreeMap::new();
        {
            info!("getting peer conf for {}", name);
            let conf_get_resp = &get_etcd_client().await?.get(key.clone(), None).await?;
            let conf_kv = &conf_get_resp.kvs()[0];
            debug_assert!(conf_kv.key_str()? == key);
            let conf_str = conf_kv.value_str()?;
            if let Value::Object(confs) = serde_json::from_str::<Value>(conf_str)? {
                for (k, v) in confs.into_iter() {
                    let v = v.as_str().ok_or(anyhow!("config key {} is not str", k))?;
                    conf_kvs.insert(k, v.to_string());
                }
            } else {
                bail!("peer is not an valid json object")
            }
        }

        let peer = PeerConfig::new(self.index, key.clone(), conf_kvs).await?;
        {
            info!("updating peer {}", name);
            peer.update().await?;
        }

        Ok(())
    }
}

pub fn is_valid_peer_name(name: &str) -> bool {
    !name.is_empty()
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains(' ')
        && name.len() < 256
}
