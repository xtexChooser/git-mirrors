use anyhow::{anyhow, bail, Result};

use crate::peer::PeerInfo;

use self::bird::BIRDConfig;

pub mod bird;

pub const KEY_ROUTE_TYPE: &str = "route_type";

pub const ROUTE_TYPE_NONE: &str = "none";
pub const ROUTE_TYPE_BIRD: &str = "bird";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RouteConfig {
    None,
    BIRD(BIRDConfig),
}

impl RouteConfig {
    pub async fn new(peer: &PeerInfo) -> Result<RouteConfig> {
        let kind = peer
            .props
            .get(KEY_ROUTE_TYPE)
            .ok_or(anyhow!("route_type not found"))?;
        let conf = match kind.as_str() {
            ROUTE_TYPE_NONE => Self::None,
            ROUTE_TYPE_BIRD => Self::BIRD(BIRDConfig::new(peer).await?),
            _ => bail!("unknown route_type: {}", kind),
        };
        Ok(conf)
    }

    pub async fn update(&self, _peer: &PeerInfo) -> Result<()> {
        match self {
            Self::None => Ok(()),
            Self::BIRD(_) => BIRDConfig::update().await,
        }
    }

    pub async fn del(&self, _peer: &PeerInfo) -> Result<()> {
        match self {
            Self::None => Ok(()),
            Self::BIRD(_) => BIRDConfig::update().await,
        }
    }
}
