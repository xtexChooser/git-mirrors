use std::mem::size_of;
use std::net::IpAddr;

use anyhow::bail;
use anyhow::Result;
use futures_util::TryStreamExt;
use tokio::task::JoinSet;
use tokio_tun::{Tun, TunBuilder};
use tracing::info;
use tracing::trace;
use tracing::warn;

use crate::config::get_config;
use crate::inet;

use self::buf::TunBuffer;
use self::ip::HandleIpExt;

pub mod buf;
pub mod ip;

pub const ERROR_HEADER_SIZE: usize = size_of::<inet::ip6_hdr>() + size_of::<inet::icmp6_hdr>();
pub const MTU: usize = 9000;
pub const BUFFER_SIZE: usize = ERROR_HEADER_SIZE + MTU;

pub async fn start_tun() -> Result<()> {
    let config = &get_config().tun;
    let tuns = TunBuilder::new()
        .name(config.ifname.as_str())
        .tap(false)
        .packet_info(false)
        .mtu(MTU as i32)
        .up()
        .try_build_mq(config.queues)
        .unwrap();

    info!(
        name = config.ifname,
        queues = tuns.len(),
        "created TUN device"
    );

    add_route(&config.ifname).await?;

    let mut tasks = JoinSet::new();
    for tun in tuns.into_iter() {
        tasks.spawn(TunHandler::new(tun)?.handle());
    }

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    Ok(())
}

async fn add_route(ifname: &str) -> Result<()> {
    let config = &get_config().addr;
    let host_addr = config.host_addr();
    info!(
        subnet = config.subnet.to_string(),
        subnet_len = config.subnet_len,
        index_len = config.index_len,
        host_addr = host_addr.to_string(),
        "adding route to TUN"
    );
    let (connection, handle, _) = rtnetlink::new_connection()?;
    tokio::spawn(connection);

    let mut links = handle.link().get().match_name(ifname.to_owned()).execute();
    let index = if let Some(link) = links.try_next().await? {
        assert!(links.try_next().await?.is_none());
        link.header.index
    } else {
        bail!("link not found")
    };
    info!(index, "got link ifindex");

    handle
        .address()
        .add(index, IpAddr::V6(host_addr), config.subnet_len)
        .execute()
        .await?;
    info!(host_addr = host_addr.to_string(), "host addr added");

    handle
        .route()
        .add()
        .v6()
        .destination_prefix(config.subnet, config.subnet_len)
        .output_interface(index)
        .pref_source(host_addr)
        .replace()
        .execute()
        .await?;
    info!("subnet route added");

    Ok(())
}

pub struct TunHandler {
    pub tun: Tun,
    pub buf: TunBuffer,
    pub recv_size: usize,
}

impl TunHandler {
    pub fn new(tun: Tun) -> Result<TunHandler> {
        let buffer = TunBuffer::new();
        Ok(Self {
            tun,
            buf: buffer,
            recv_size: 0,
        })
    }

    pub async fn handle(mut self) -> Result<()> {
        info!("handling TUN");
        loop {
            let size = self.tun.recv(self.buf.read_buffer()).await?;
            self.recv_size = size;
            if let Err(e) = self.handle_packet().await {
                warn!(err = e.to_string(), size, "failed to handle a packet");
            }
        }
    }

    pub async fn handle_packet(&mut self) -> Result<()> {
        if self.recv_size <= size_of::<inet::ip6_hdr>() {
            trace!(
                size = self.recv_size,
                required = size_of::<inet::ip6_hdr>(),
                "received a pkt which is smaller than a IPv6 header"
            )
        } else {
            self.handle_ip().await?;
        }
        Ok(())
    }
}
