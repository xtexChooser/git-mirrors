use anyhow::{anyhow, bail, Result};
use futures::TryStreamExt;
use netlink_packet_generic::GenlMessage;
use netlink_packet_route::{
    nlas::link::{Info, InfoKind, Nla},
    NetlinkMessage, NetlinkPayload, NLM_F_ACK, NLM_F_REQUEST,
};
use netlink_packet_wireguard::{nlas::WgDeviceAttrs, Wireguard, WireguardCmd};
use std::net::{SocketAddr, ToSocketAddrs};

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
                        endpoint
                            .as_str()
                            .to_socket_addrs()
                            .map_err(|e| anyhow!("failed to resolve WG endpoint: {}", e))?
                            .as_mut_slice(),
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

    pub async fn update(&self, peer: &PeerInfo) -> Result<()> {
        let (connection, handle, _) =
            rtnetlink::new_connection().map_err(|e| anyhow!("failed to connect to rtnl: {}", e))?;
        tokio::spawn(connection);
        let ifname = to_ifname(peer).await?;
        info!("updating WG if '{}'", ifname);

        let mut ifgetreq = handle.link().get().match_name(ifname.clone()).execute();
        if ifgetreq.try_next().await?.is_none() {
            info!("WG if '{}' not found, adding", ifname);
            let mut ifaddreq = handle.link().add();
            ifaddreq
                .message_mut()
                .nlas
                .push(Nla::IfName(ifname.clone()));
            ifaddreq
                .message_mut()
                .nlas
                .push(Nla::Info(vec![Info::Kind(InfoKind::Wireguard)]));
            ifaddreq.execute().await?;
        }
        debug_assert!(ifgetreq.try_next().await?.is_none());

        let (connection, mut handle, _) =
            genetlink::new_connection().map_err(|e| anyhow!("failed to connect to genl: {}", e))?;
        tokio::spawn(connection);

        let wg_genlmsg = GenlMessage::from_payload(Wireguard {
            cmd: WireguardCmd::SetDevice,
            nlas: vec![WgDeviceAttrs::IfName(ifname.clone())],
        });
        let mut wg_nlmsg = NetlinkMessage::from(wg_genlmsg);
        wg_nlmsg.header.flags = NLM_F_REQUEST | NLM_F_ACK;
        let mut wg_req = handle.request(wg_nlmsg).await?;

        let wg_resp = wg_req
            .try_next()
            .await?
            .ok_or(anyhow!("WG set req get no ACK"))?;
        if let NetlinkPayload::Ack(ack) = wg_resp.payload {
            if ack.code != 0 {
                bail!("WG set req failed, ACK code: {}", ack.code);
            }
        } else {
            bail!("WG set req get not ACK");
        }
        debug_assert!(wg_req.try_next().await?.is_none());

        Ok(())
    }

    pub async fn del(&self, peer: &PeerInfo) -> Result<()> {
        let ifname = to_ifname(peer).await?;
        let (connection, handle, _) =
            rtnetlink::new_connection().map_err(|e| anyhow!("failed to connect to rtnl: {}", e))?;
        tokio::spawn(connection);
        info!("deleting WG if '{}'", ifname);
        let mut ifreq = handle.link().get().match_name(ifname.clone()).execute();
        if let Some(ifinfo) = ifreq.try_next().await? {
            handle.link().del(ifinfo.header.index);
        } else {
            warn!("WG if with name '{}' not found", ifname);
        }
        debug_assert!(ifreq.try_next().await?.is_none());
        Ok(())
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

pub async fn select_endpoint(endpoints: &mut [SocketAddr]) -> Result<SocketAddr> {
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
        score.cmp(&0)
    });
    Ok(endpoints
        .last()
        .ok_or(anyhow!("no WG endpoints found"))?
        .to_owned())
}

pub async fn delete_unknown_if() -> Result<()> {
    let (connection, handle, _) =
        rtnetlink::new_connection().map_err(|e| anyhow!("failed to connect to rtnl: {}", e))?;
    tokio::spawn(connection);
    let mut links = handle.link().get().execute();
    while let Some(link) = links.try_next().await? {
        let ifindex = link.header.index;
        let mut ifname: Option<String> = None;
        for nla in link.nlas.into_iter() {
            if let Nla::IfName(name) = nla {
                ifname = Some(name);
                break;
            }
        }
        if let Some(ifname) = ifname {
            for zone in &config::get_config().await?.zone {
                if let Some(wg) = &zone.wireguard && ifname.starts_with(&wg.ifname_prefix) {
                        info!("find if '{}'({}) with the wg ifname prefix of zone {}", ifname, ifindex, zone.name);
                        let mut found = false;
                        for peer in &zone.peers {
                            if matches!(&peer.tun, TunnelConfig::WireGuard(_)) && to_ifname(&peer.info).await? == ifname{
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
