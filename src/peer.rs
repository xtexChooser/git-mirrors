use std::collections::BTreeMap;

use anyhow::Result;

use crate::{tunnel::TunnelConfig, zone::Zone};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PeerInfo {
    pub zone: &'static Zone,
    pub name: String,
    pub props: BTreeMap<String, String>,
}

impl PeerInfo {
    pub fn new(
        zone: &'static Zone,
        name: String,
        props: BTreeMap<String, String>,
    ) -> Result<PeerInfo> {
        let conf = PeerInfo { zone, name, props };
        Ok(conf)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PeerConfig {
    pub info: PeerInfo,
    pub tun: TunnelConfig,
}

impl PeerConfig {
    pub fn new(zone: &'static Zone, name: String, props: BTreeMap<String, String>) -> Result<Self> {
        Self::try_from(PeerInfo::new(zone, name, props)?)
    }

    pub async fn create(&self) -> Result<()> {
        self.tun.create(&self.info).await?;
        Ok(())
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

impl TryFrom<PeerInfo> for PeerConfig {
    type Error = anyhow::Error;

    fn try_from(info: PeerInfo) -> Result<Self> {
        let tun = TunnelConfig::new(&info)?;
        let conf = PeerConfig { info, tun };
        Ok(conf)
    }
}
