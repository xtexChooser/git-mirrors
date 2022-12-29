use anyhow::{anyhow, Result};
use futures::TryStreamExt;
use netlink_packet_route::nlas::link::Nla;
use rtnetlink::new_connection;
use std::{
    cmp::Ordering,
    net::{SocketAddr, ToSocketAddrs},
};

use crate::{config, peer::PeerInfo, tunnel::TunnelConfig};

pub const KEY_ENDPOINT: &str = "wg_endpoint";
pub const KEY_PRIVATE_KEY: &str = "wg_private_key";
pub const KEY_LISTEN_PORT: &str = "wg_listen_port";
pub const KEY_FORWARD_MARK: &str = "wg_fw_mark";
pub const KEY_PUBLIC_KEY: &str = "wg_remote_public_key";
pub const KEY_PRESHARED_KEY: &str = "wg_preshared_key";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WireGuardConfig {
    endpoint: Option<SocketAddr>,
    private_key: String,
    listen_port: Option<u16>,
    fw_mark: Option<u32>,
    remote_public_key: String,
    preshared_key: Option<String>,
}

impl WireGuardConfig {
    pub async fn new(peer: &PeerInfo) -> Result<WireGuardConfig> {
        let conf = WireGuardConfig {
            endpoint: if let Some(endpoint) = peer.props.get(KEY_ENDPOINT) {
                Some(
                    select_endpoint(
                        &mut endpoint
                            .as_str()
                            .to_socket_addrs()
                            .map_err(|e| anyhow!("failed to resolve WG endpoint: {}", e))?
                            .collect(),
                    )
                    .await?,
                )
            } else {
                None
            },
            private_key: peer
                .props
                .get(KEY_PRIVATE_KEY)
                .ok_or(anyhow!("no WG priv key"))?
                .to_owned(),
            listen_port: if let Some(port) = peer.props.get(KEY_LISTEN_PORT) {
                Some(port.as_str().parse()?)
            } else {
                None
            },
            fw_mark: if let Some(fwmark) = peer.props.get(KEY_FORWARD_MARK) {
                Some(fwmark.as_str().parse()?)
            } else {
                None
            },
            remote_public_key: peer
                .props
                .get(KEY_PUBLIC_KEY)
                .ok_or(anyhow!("no WG pub key"))?
                .to_owned(),
            preshared_key: if let Some(preshared) = peer.props.get(KEY_PRESHARED_KEY) {
                Some(preshared.as_str().parse()?)
            } else {
                None
            },
        };
        Ok(conf)
    }

    pub fn update(&self, _peer: &PeerInfo) -> Result<()> {
        todo!();
    }

    pub fn del(&self, _peer: &PeerInfo) -> Result<()> {
        todo!();
    }
}

pub async fn get_config() -> Result<config::WireGuardConfig> {
    Ok(config::get_config()
        .await?
        .wireguard
        .as_ref()
        .ok_or(anyhow!("no WG configured"))?
        .to_owned())
}

pub async fn to_ifname(peer: &PeerInfo) -> Result<String> {
    let mut name = peer.name.to_owned();
    if get_config().await?.crc_if_peer_name {
        name = format!("{:x}", crc32fast::hash(name.as_bytes()));
    }
    let ifname_prefix = &peer
        .zone
        .wireguard
        .as_ref()
        .ok_or(anyhow!("no WG configured for zone"))?
        .ifname_prefix;
    Ok(format!("{ifname_prefix}{name}"))
}

pub async fn select_endpoint(endpoints: &mut Vec<SocketAddr>) -> Result<SocketAddr> {
    let prefer_ipv6 = get_config().await?.prefer_ipv6;
    endpoints.sort_by(|p1, p2| {
        let mut score = 0;
        if prefer_ipv6 {
            if p1.is_ipv6() {
                score += 100;
            }
            if p2.is_ipv6() {
                score -= 100;
            }
        } else {
            if p1.is_ipv4() {
                score += 100;
            }
            if p2.is_ipv4() {
                score -= 100;
            }
        }
        if score == 0 {
            Ordering::Equal
        } else if score > 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    Ok(endpoints
        .last()
        .ok_or(anyhow!("no WG endpoints found"))?
        .to_owned())
}

pub async fn delete_unknown_if() -> Result<()> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);
    let mut links = handle.link().get().execute();
    while let Some(link) = links.try_next().await? {
        let ifindex = link.header.index;
        let mut ifname: Option<String> = None;
        for nla in link.nlas.into_iter() {
            match nla {
                Nla::IfName(name) => ifname = Some(name),
                _ => (),
            }
        }
        if let Some(ifname) = ifname {
            for zone in &config::get_config().await?.zone {
                if let Some(wg) = &zone.wireguard && ifname.starts_with(&wg.ifname_prefix) {
                        info!("find if '{}'({}) with the wg ifname prefix of zone {}", ifname, ifindex, zone.name);
                        let mut found = false;
                        for peer in &zone.peers {
                            if let TunnelConfig::WireGuard(_) = &peer.tun && to_ifname(&peer.info).await? == ifname{
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            warn!("if {} found but no peer info recorded, trying to remove", ifindex);
                            handle.link().del(ifindex).execute().await?;
                        }
                    }
            }
        }
    }
    Ok(())
}
