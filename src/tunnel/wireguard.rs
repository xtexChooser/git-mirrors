use anyhow::{anyhow, bail, Result};
use cidr::{IpInet, Ipv4Inet, Ipv6Inet};
use futures::TryStreamExt;
use netlink_packet_generic::GenlMessage;
use netlink_packet_route::{
    address, link,
    nlas::link::{Info, InfoKind},
    NetlinkMessage, AF_INET6, NLM_F_ACK, NLM_F_REQUEST,
};
use netlink_packet_wireguard::{
    constants::{AF_INET, WGDEVICE_F_REPLACE_PEERS, WGPEER_F_REPLACE_ALLOWEDIPS},
    nlas::{WgAllowedIp, WgAllowedIpAttrs, WgDeviceAttrs, WgPeer, WgPeerAttrs},
    Wireguard, WireguardCmd,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};

use crate::{config, peer::PeerInfo, tunnel::TunnelConfig, zone};

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
        let ifname = to_ifname(peer).await?;
        info!("updating WG if '{}'", ifname);

        let (connection, rtnl, _) =
            rtnetlink::new_connection().map_err(|e| anyhow!("failed to connect to rtnl: {}", e))?;
        tokio::spawn(connection);

        let mut ifgetreq = rtnl.link().get().match_name(ifname.clone()).execute();
        let ifindex = if let Some(Some(ifinfo)) = ifgetreq.try_next().await.ok() {
            ifinfo.header.index
        } else {
            info!("WG if '{}' not found, adding", ifname);
            let mut ifaddreq = rtnl.link().add();
            let nlas = &mut ifaddreq.message_mut().nlas;
            nlas.push(link::nlas::Nla::IfName(ifname.clone()));
            nlas.push(link::nlas::Nla::Info(vec![Info::Kind(InfoKind::Wireguard)]));
            ifaddreq.execute().await?;

            let mut ifgetreq2 = rtnl.link().get().match_name(ifname.clone()).execute();
            if let Some(ifinfo) = ifgetreq2.try_next().await? {
                ifinfo.header.index
            } else {
                bail!(
                    "if with name {} added successfully, but could not get back",
                    ifname
                )
            }
        };
        debug_assert!(ifgetreq.try_next().await?.is_none());

        {
            let (connection, mut genl, _) = genetlink::new_connection()
                .map_err(|e| anyhow!("failed to connect to genl: {}", e))?;
            tokio::spawn(connection);

            let mut wg_peer_nlas = vec![
                WgPeerAttrs::PublicKey(decode_key(self.remote_public_key.as_str())?),
                WgPeerAttrs::AllowedIps(vec![
                    WgAllowedIp(vec![
                        WgAllowedIpAttrs::Family(AF_INET),
                        WgAllowedIpAttrs::IpAddr(IpAddr::V4("0.0.0.0".parse::<Ipv4Addr>()?)),
                        WgAllowedIpAttrs::Cidr(0),
                    ]),
                    WgAllowedIp(vec![
                        WgAllowedIpAttrs::Family(AF_INET6),
                        WgAllowedIpAttrs::IpAddr(IpAddr::V6("::".parse::<Ipv6Addr>()?)),
                        WgAllowedIpAttrs::Cidr(0),
                    ]),
                ]),
                WgPeerAttrs::Flags(WGPEER_F_REPLACE_ALLOWEDIPS),
            ];
            if let Some(psk) = &self.preshared_key {
                wg_peer_nlas.push(WgPeerAttrs::PresharedKey(decode_key(psk.as_str())?));
            }
            if let Some(endpoint) = &self.endpoint {
                wg_peer_nlas.push(WgPeerAttrs::Endpoint(endpoint.to_owned()));
            }
            if let Some(endpoint) = &self.endpoint {
                wg_peer_nlas.push(WgPeerAttrs::Endpoint(endpoint.to_owned()));
            }
            //   @TODO: PersistentKeepalive(u16),

            let wg_nlas = vec![
                WgDeviceAttrs::IfIndex(ifindex),
                WgDeviceAttrs::PrivateKey(decode_key(self.private_key.as_str())?),
                WgDeviceAttrs::ListenPort(self.listen_port.unwrap_or(0)),
                WgDeviceAttrs::Fwmark(self.fw_mark.unwrap_or(0)),
                WgDeviceAttrs::Flags(WGDEVICE_F_REPLACE_PEERS),
                WgDeviceAttrs::Peers(vec![WgPeer(wg_peer_nlas)]),
            ];

            let wg_genlmsg = GenlMessage::from_payload(Wireguard {
                cmd: WireguardCmd::SetDevice,
                nlas: wg_nlas,
            });
            let mut wg_nlmsg = NetlinkMessage::from(wg_genlmsg);
            wg_nlmsg.header.flags = NLM_F_REQUEST | NLM_F_ACK;
            let mut wg_req = genl.request(wg_nlmsg).await?;

            // @TODO: https://github.com/rust-netlink/netlink-proto/pull/4
            /*let wg_resp = wg_req
                .try_next()
                .await?
                .ok_or(anyhow!("WG set req get no ACK"))?;
            let wg_resp_payload = wg_resp.payload;
            if let NetlinkPayload::Error(err) = wg_resp_payload {
                bail!("WG set req failed, err code: {}", err.code);
            } else if !matches!(wg_resp_payload, NetlinkPayload::Ack(_)) {
                bail!("WG set req get not ACK and not err");
            }*/
            debug_assert!(wg_req.try_next().await?.is_none());
        }
        {
            let expected_prefixes = &peer.get_zone().parsed_ip_prefixes;
            let mut addrs = rtnl
                .address()
                .get()
                .set_link_index_filter(ifindex)
                .execute();
            let mut exist_addrs = vec![];
            'outer: while let Some(addr_msg) = addrs.try_next().await? {
                let mut addr = None;
                for nla in &addr_msg.nlas {
                    if let address::Nla::Address(addr_vec) = nla {
                        addr = Some(match addr_msg.header.family.into() {
                            AF_INET => IpInet::V4(Ipv4Inet::new(
                                Ipv4Addr::from(TryInto::<[u8; 4]>::try_into(addr_vec.as_slice())?),
                                addr_msg.header.prefix_len,
                            )?),
                            AF_INET6 => IpInet::V6(Ipv6Inet::new(
                                Ipv6Addr::from(TryInto::<[u8; 16]>::try_into(addr_vec.as_slice())?),
                                addr_msg.header.prefix_len,
                            )?),
                            _ => continue 'outer,
                        });
                    }
                }
                let addr = addr.unwrap();
                exist_addrs.push(addr.clone());
                if !expected_prefixes.contains(&addr) {
                    rtnl.address().del(addr_msg).execute().await?;
                }
            }
            for addr in expected_prefixes {
                if !exist_addrs.contains(addr) {
                    rtnl.address()
                        .add(ifindex, addr.address(), addr.network_length())
                        .execute()
                        .await?;
                }
            }
        }

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
            handle.link().del(ifinfo.header.index).execute().await?;
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
    {
        if get_config().await?.crc_if_peer_name {
            name = format!("{:x}", crc32fast::hash(name.as_bytes()));
        }
    }
    let zone = peer.get_zone();
    let ifname_prefix = &zone
        .conf
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

pub fn decode_key(key: &str) -> Result<[u8; 32]> {
    base64::decode(key)?
        .try_into()
        .map_err(|e| anyhow!("incorrect decoded WG key len: {:?}", e))
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
            if let link::nlas::Nla::IfName(name) = nla {
                ifname = Some(name);
                break;
            }
        }
        if let Some(ifname) = ifname {
            for zone in zone::get_zones() {
                if let Some(wg) = &zone.conf.wireguard && ifname.starts_with(&wg.ifname_prefix) {
                    let mut found = false;
                    let peers = zone.peers.lock().await;
                    for peer in peers.iter() {
                        if matches!(&peer.tun, TunnelConfig::WireGuard(_)) && to_ifname(&peer.info).await? == ifname {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        warn!("if {} found but no peer info is recorded, trying to remove", ifindex);
                        handle.link().del(ifindex).execute().await?;
                    }
                }
            }
        }
    }
    Ok(())
}
