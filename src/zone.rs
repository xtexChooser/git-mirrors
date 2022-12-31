use std::{
    collections::{BTreeMap, VecDeque},
    slice::IterMut,
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Ok, Result};

use cidr::IpInet;
use etcd_client::GetOptions;
use serde::Deserialize;
use serde_json::Value;
use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    config::get_config, etcd::get_etcd_client, peer::PeerConfig, util::soft_err::SoftError, watcher,
};

pub static mut ZONES: Vec<Zone> = Vec::new();

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ZoneConfig {
    pub name: String,
    pub etcd_prefix: String,
    pub ip_prefixes: Vec<String>,
    pub wireguard: Option<WireGuardConfig>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    pub ifname_prefix: String,
}

#[derive(Debug)]
pub struct Zone {
    pub conf: ZoneConfig,
    pub index: usize,
    pub peers: Mutex<Vec<PeerConfig>>,
    pub parsed_ip_prefixes: Vec<IpInet>,
}

impl PartialEq for Zone {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for Zone {}

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
    let mut parsed_ip_prefixes = vec![];
    for ip_prefix in &conf.ip_prefixes {
        parsed_ip_prefixes
            .push(IpInet::from_str(ip_prefix.as_str()).with_context(|| ip_prefix.to_owned())?);
    }
    let zone = Zone {
        conf,
        index: unsafe { ZONES.len() },
        peers: Mutex::new(vec![]),
        parsed_ip_prefixes,
    };
    unsafe {
        ZONES.push(zone);
    }
    let zone = unsafe { ZONES.last_mut() }.unwrap();
    zone.sync_all_peers().await?;
    Ok(())
}

pub async fn watch_zones() -> Result<Vec<JoinHandle<()>>> {
    let mut iter = unsafe { ZONES.iter_mut() };
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    while let Some(zone) = iter.next() {
        let name = zone.conf.name.to_owned();
        handles.push(tokio::spawn(async move {
            if let Err(err) = watcher::watch_zone(zone).await {
                error!("error watching changes in zone {}: {}", name, err)
            }
            warn!("watcher for {} stopped", name)
        }));
    }
    Ok(handles)
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
                if err.downcast_ref::<SoftError>().is_some() {
                    error!(
                        "failed to import peer {} in {}, fail soft: {}",
                        key, zone_name, err
                    );
                } else {
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
                    etcd.put(prefix.clone() + r"_" + &key, val, None).await?;
                }
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

        let peer = PeerConfig::new(self.index, name.to_string(), conf_kvs).await?;
        {
            let mut peers = self.peers.lock().await;
            let peer = {
                let mut index = 0;
                let mut found = false;
                for exist_peer in peers.iter() {
                    if exist_peer == &peer {
                        found = true;
                        break;
                    }
                    index += 1;
                }
                if found {
                    peers.remove(index);
                }
                peers.push(peer);
                peers.last().unwrap()
            };
            info!("updating peer {}", name);
            SoftError::wrap(peer.update().await)?;
        }

        Ok(())
    }

    pub async fn remove_peer(&mut self, name: &str) -> Result<()> {
        if name.starts_with('_') {
            return Ok(());
        }

        let mut peers = self.peers.lock().await;
        let mut index = 0;
        let mut found = false;
        for peer in peers.iter() {
            if peer.info.name.as_str() == name {
                found = true;
                break;
            }
            index += 1;
        }
        if found {
            let peer = &peers[index];
            let res = peer.del().await;
            peers.remove(index);
            res?;
        } else {
            bail!("peer with the given name not found")
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
