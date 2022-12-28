use anyhow::Result;
use futures::TryStreamExt;
use netlink_packet_route::nlas::link::{Info, InfoKind, Nla};
use rtnetlink::new_connection;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::peer::PeerConfig;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct WireGuardConfig {
    endpoint: Option<SocketAddr>,
    private_key: String,
    listen_port: Option<u16>,
    fw_mark: Option<u32>,
    remote_public_key: String,
    preshared_key: Option<String>,
}

impl WireGuardConfig {
    pub async fn create(self: &Self, peer: &PeerConfig) -> Result<()> {
        Ok(())
    }

    pub async fn delete(self: &Self, peer: &PeerConfig) -> Result<()> {
        Ok(())
    }

    pub async fn update(self: &Self, peer: &PeerConfig) -> Result<()> {
        Ok(())
    }

    pub async fn delete_unknown_if() -> Result<()> {
        let (connection, handle, _) = new_connection().unwrap();
        tokio::spawn(connection);
        let mut links = handle.link().get().execute();
        while let Some(link) = links.try_next().await? {
            let mut name: Option<String> = None;
            let mut kind: Option<InfoKind> = None;
            for nla in link.nlas.into_iter() {
                match nla {
                    Nla::IfName(ifname) => name = Some(ifname),
                    Nla::Info(infos) => {
                        for info in infos.into_iter() {
                            match info {
                                Info::Kind(ifkind) => kind = Some(ifkind),
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
            if let Some(name) = name && let Some(kind) = kind {
                if kind == InfoKind::Wireguard {
                    info!("find wg if '{}'", name);
                }
            }
        }
        Ok(())
    }
}
