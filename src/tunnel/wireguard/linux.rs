use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use anyhow::{Context, Ok, Result};
use wireguard_uapi::{
    set::{AllowedIp, Device, Peer, WgDeviceF},
    RouteSocket, WgSocket,
};

use crate::{
    peer_conf::tunnel::WGTunnelConfig,
    tunnel::{TunManager, Tunnel},
};

use super::parse_wg_key;

pub static WG_LINUX_TUN_MANAGER: WGLinuxTunManager = WGLinuxTunManager {};
pub static WG_ALLOWED_IPS: [AllowedIp; 2] = [
    AllowedIp {
        ipaddr: &IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        cidr_mask: Some(0),
    },
    AllowedIp {
        ipaddr: &IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)),
        cidr_mask: Some(0),
    },
];

#[derive(Debug)]
pub struct WGLinuxTunManager {}

impl WGLinuxTunManager {
    fn connect_route_socket() -> Result<RouteSocket> {
        Ok(RouteSocket::connect().context("connect to WG route socket")?)
    }

    fn connect_wg_socket() -> Result<WgSocket> {
        Ok(WgSocket::connect().context("connect to WG control socket")?)
    }
}

impl TunManager<WGTunnelConfig> for WGLinuxTunManager {
    fn list(&self) -> Result<Vec<String>> {
        Ok(Self::connect_route_socket()?
            .list_device_names()
            .context("list WG device names")?)
    }

    fn add(&self, tun: &Tunnel<WGTunnelConfig>) -> Result<()> {
        Self::connect_route_socket()?
            .add_device(tun.to_qualified_name().as_str())
            .context("add new WG device")?;
        self.update(tun)?;
        Ok(())
    }

    fn update(&self, tun: &Tunnel<WGTunnelConfig>) -> Result<()> {
        let private_key = parse_wg_key(tun.config.private_key.as_str())?;
        let public_key = parse_wg_key(tun.config.public_key.as_str())?;

        // @TODO: replace with WG_ALLOWED_IPS when wireguard-uapi-rs#27 released
        let mut peer = Peer::from_public_key(&public_key).allowed_ips(vec![
            AllowedIp {
                ipaddr: &WG_ALLOWED_IPS[0].ipaddr,
                cidr_mask: Some(0),
            },
            AllowedIp {
                ipaddr: &WG_ALLOWED_IPS[1].ipaddr,
                cidr_mask: Some(0),
            },
        ]);
        if tun.config.keep_alive.is_some() {
            peer = peer.persistent_keepalive_interval(tun.config.keep_alive.unwrap())
        }
        let endpoint_addr: SocketAddr;
        if let Some(endpoint_str) = &tun.config.endpoint {
            endpoint_addr = SocketAddr::parse_ascii(endpoint_str.as_str().as_bytes())
                .context("parse endpoint address for WG tunnel")?;
            peer = peer.endpoint(&endpoint_addr);
        }
        let preshared_key: [u8; 32];
        if let Some(preshared_key_str) = &tun.config.preshared_key {
            preshared_key = parse_wg_key(preshared_key_str.as_str())?;
            peer = peer.preshared_key(&preshared_key);
        }

        let mut device = Device::from_ifname(tun.name.as_str())
            .flags(vec![WgDeviceF::ReplacePeers])
            .private_key(&private_key)
            .peers(vec![peer]);
        if tun.config.listen_port.is_some() {
            device = device.listen_port(tun.config.listen_port.unwrap())
        }
        if tun.config.fwmark.is_some() {
            device = device.fwmark(tun.config.fwmark.unwrap())
        }
        Self::connect_wg_socket()?
            .set_device(device)
            .context("set WG device info")?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<()> {
        Self::connect_route_socket()?
            .del_device(name)
            .context(format!("remove ifname {}", name))?;
        Ok(())
    }
}
