use anyhow::{Ok, Result};
use serde::Deserialize;

use crate::{
    config::{tunnel::wireguard::WGBackend, CONFIG},
    tunnel::{wireguard::linux::WG_LINUX_TUN_MANAGER, TunManager, TunnelConfig},
};

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash, Default)]
pub struct PeerTunnelConfig {
    #[serde(default)]
    pub wireguard: Vec<WGTunnelConfig>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WGTunnelConfig {
    pub listen_port: Option<u16>,
    pub private_key: String,
    pub public_key: String,
    pub preshared_key: Option<String>,
    pub endpoint: Option<String>,
    pub keep_alive: Option<u16>,
    pub fwmark: Option<u32>,
}

impl TunnelConfig for WGTunnelConfig {
    fn get_manager(&self) -> Result<&'static dyn TunManager<Self>> {
        match CONFIG
            .tunnel
            .wireguard
            .as_ref()
            .expect("WG not configureg")
            .backend
        {
            WGBackend::LINUX => Ok(&WG_LINUX_TUN_MANAGER),
            WGBackend::XPLATFORM => todo!(),
        }
    }
}
