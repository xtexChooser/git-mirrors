use crate::{
    peer_conf::tunnel::WGTunnelConfig,
    tunnel::{TunManager, Tunnel},
};

pub static WG_LINUX_TUN_MANAGER: WGLinuxTunManager = WGLinuxTunManager {};

#[derive(Debug)]
pub struct WGLinuxTunManager {}

impl TunManager<WGTunnelConfig> for WGLinuxTunManager {
    fn list(&self) -> anyhow::Result<Vec<String>> {
        todo!()
    }

    fn add(&mut self, tun: &Tunnel<WGTunnelConfig>) -> anyhow::Result<()> {
        todo!()
    }

    fn update(&mut self, tun: &Tunnel<WGTunnelConfig>) -> anyhow::Result<()> {
        todo!()
    }

    fn remove(&mut self, name: &str) -> anyhow::Result<()> {
        todo!()
    }
}
