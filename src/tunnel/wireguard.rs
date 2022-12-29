use anyhow::Result;
use futures::TryStreamExt;
use netlink_packet_route::nlas::link::{Info, InfoKind, Nla};
use rtnetlink::new_connection;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::{config::get_config, peer::PeerConfig, tunnel::TunnelConfig};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct WireGuardConfig {
    #[serde(skip)]
    peer: Option<*const PeerConfig>,
    endpoint: Option<SocketAddr>,
    private_key: String,
    listen_port: Option<u16>,
    fw_mark: Option<u32>,
    remote_public_key: String,
    preshared_key: Option<String>,
}

impl WireGuardConfig {
    pub fn link(self: &mut Self, peer: *const PeerConfig) {
        self.peer = Some(peer);
    }

    pub async fn create(self: &Self, peer: &PeerConfig) -> Result<()> {
        Ok(())
    }

    pub async fn delete(self: &Self, peer: &PeerConfig) -> Result<()> {
        Ok(())
    }

    pub async fn update(self: &Self, peer: &PeerConfig) -> Result<()> {
        Ok(())
    }

    pub fn to_ifname(self: &Self) -> Result<String> {
        /*if get_config()?.wireguard.unwrap().crc_if_peer_name {
            Ok()
        }*/
        Ok("".to_string())
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
                for zone in &get_config()?.zone {
                    if let Some(wg) = &zone.wireguard && ifname.starts_with(&wg.ifname_prefix) {
                        info!("find if '{}'({}) with the wg ifname prefix of zone {}", ifname, ifindex, zone.name);
                        let mut found = false;
                        for peer in &zone.peers {
                            if let TunnelConfig::WireGuard(cfg) = &peer.tun && cfg.to_ifname()? == ifname{
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
}
