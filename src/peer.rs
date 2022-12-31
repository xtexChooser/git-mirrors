use std::collections::BTreeMap;

use anyhow::Result;

use crate::{tunnel::TunnelConfig, zone::Zone};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PeerInfo {
    pub zone: usize,
    pub name: String,
    pub props: BTreeMap<String, String>,
}

impl PeerInfo {
    pub fn new(zone: usize, name: String, props: BTreeMap<String, String>) -> Result<PeerInfo> {
        let conf = PeerInfo { zone, name, props };
        Ok(conf)
    }

    pub fn get_zone(&self) -> &'static Zone {
        Zone::get(self.zone)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PeerConfig {
    pub info: PeerInfo,
    pub tun: TunnelConfig,
}

impl PeerConfig {
    pub async fn new(zone: usize, name: String, props: BTreeMap<String, String>) -> Result<Self> {
        Self::try_from(PeerInfo::new(zone, name, props)?).await
    }

    pub async fn try_from(info: PeerInfo) -> Result<Self> {
        let tun = TunnelConfig::new(&info).await?;
        let conf = PeerConfig { info, tun };
        Ok(conf)
    }

    pub async fn update(&self) -> Result<()> {
        self.tun.update(&self.info).await?;
        Ok(())
    }

    pub async fn del(&self) -> Result<()> {
        self.tun.del(&self.info).await?;
        Ok(())
    }
}
