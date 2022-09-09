use anyhow::Result;

use crate::zone::Zone;

pub mod wireguard;

pub trait TunnelConfig: Sized {
    fn get_manager(&self) -> Result<&'static dyn TunManager<Self>>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Tunnel<CONF: TunnelConfig> {
    pub name: String,
    pub zone: Zone,
    pub config: Box<CONF>,
}

impl<CONF: TunnelConfig> Tunnel<CONF> {
    pub fn get_manager(&self) -> Result<&'static dyn TunManager<CONF>> {
        self.config.get_manager()
    }
    pub fn to_qualified_name(&self) -> String {
        String::from("_")
    }
}

pub trait TunManager<CONF: TunnelConfig> {
    fn list(&self) -> Result<Vec<String>>;
    fn add(&mut self, tun: &Tunnel<CONF>) -> Result<()>;
    fn update(&mut self, tun: &Tunnel<CONF>) -> Result<()>;
    fn remove(&mut self, name: &str) -> Result<()>;
}
