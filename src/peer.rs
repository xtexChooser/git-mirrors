use std::collections::BTreeMap;

use anyhow::Result;

use crate::{route::RouteConfig, tunnel::TunnelConfig, zone::Zone};

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

#[derive(Debug, Clone)]
pub struct PeerConfig {
    pub info: PeerInfo,
    pub tun: TunnelConfig,
    pub route: RouteConfig,
}

impl PartialEq for PeerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}
impl Eq for PeerConfig {}

impl PeerConfig {
    pub async fn new(zone: usize, name: String, props: BTreeMap<String, String>) -> Result<Self> {
        Self::try_from(PeerInfo::new(zone, name, props)?).await
    }

    pub async fn try_from(info: PeerInfo) -> Result<Self> {
        let tun = TunnelConfig::new(&info).await?;
        let route = RouteConfig::new(&info).await?;
        let conf = PeerConfig { info, tun, route };
        Ok(conf)
    }

    pub async fn update(&self) -> Result<()> {
        self.tun.update(&self.info).await?;
        self.route.update(&self.info).await?;
        Ok(())
    }

    pub async fn del(&self) -> Result<()> {
        self.tun.del(&self.info).await?;
        self.route.del(&self.info).await?;
        Ok(())
    }

    #[inline]
    pub async fn get_ifname(&self) -> Result<Option<String>> {
        self.tun.get_ifname(&self.info).await
    }
}
