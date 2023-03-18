use std::borrow::BorrowMut;
use std::mem::size_of;
use std::ops::DerefMut;

use anyhow::Result;
use tokio::task::JoinSet;
use tokio_tun::{Tun, TunBuilder};
use tracing::info;
use tracing::trace;

use crate::config::get_config;
use crate::inet;
use crate::inet::icmphdr;
use crate::inet::ip;

use self::buf::TunBuffer;

pub mod buf;

pub const ERROR_HEADER_SIZE: usize = size_of::<ip>() + size_of::<icmphdr>();
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

    let mut tasks = JoinSet::new();
    for tun in tuns.into_iter() {
        tasks.spawn(TunHandler::new(tun)?.handle());
    }

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    Ok(())
}

pub struct TunHandler {
    pub tun: Tun,
    pub buffer: TunBuffer,
}

unsafe impl Send for TunHandler {}
unsafe impl Sync for TunHandler {}

impl TunHandler {
    pub fn new(tun: Tun) -> Result<TunHandler> {
        let buffer = TunBuffer::new();
        Ok(Self { tun, buffer })
    }

    pub async fn handle(mut self) -> Result<()> {
        info!("handling TUN");
        let buffer = self.buffer.read_buffer();
        loop {
            let size = self.tun.recv(buffer).await?;
            if size <= size_of::<inet::ip6_hdr>() {
                trace!(size, "received a pkt which is smaller than a IPv6 header")
            }
            /*if size <= size_of::<netinet::ip>() {
                self.log(format!("Received packet with too small size {}", size));
            } else {
                handle_ip(&self).await?;
            }*/
        }
        Ok(())
    }
}
