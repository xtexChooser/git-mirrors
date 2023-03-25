use std::{
    cmp::{max, min},
    mem::size_of,
    net::Ipv6Addr,
};

use anyhow::{bail, Result};

use crate::{
    inet,
    resolver::resolve,
    subnet::{self, SubnetConfig},
};

use super::{
    buf::TunBuffer,
    ip::{self},
    TunHandler,
};

pub trait Icmp6Handler {
    async fn handle_icmpv6(
        &mut self,
        src: Ipv6Addr,
        dst: Ipv6Addr,
        subnet: &SubnetConfig,
        index: u128,
        hop: u8,
    ) -> Result<()>;
}

impl Icmp6Handler for TunHandler {
    async fn handle_icmpv6(
        &mut self,
        src: Ipv6Addr,
        dst: Ipv6Addr,
        subnet: &SubnetConfig,
        index: u128,
        hop: u8,
    ) -> Result<()> {
        if let Some(chain) = resolve(index).await {
            let ip6 = self.buf.read_object::<inet::ip6_hdr>(0);
            let hops = chain.len();
            if hop >= hops {
                // ADDR UNREACHABLE for hops out of range
                send_icmpv6_error_reply(
                    self,
                    &subnet.with_hop(dst, hops - 1),
                    &src,
                    inet::ICMP6_DST_UNREACH,
                    inet::ICMP6_DST_UNREACH_ADDR,
                    0,
                )
                .await?;
            } else {
                let ttl = unsafe { ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim };
                println!("hop {}/{} ttl {}", hop, hops, ttl);
                if hops - hop > ttl {
                    // TTL EXCEEDED
                    send_icmpv6_error_reply(
                        self,
                        &subnet.with_hop(dst, hops - ttl),
                        &src,
                        inet::ICMP6_TIME_EXCEEDED,
                        inet::ICMP6_TIME_EXCEED_TRANSIT,
                        0,
                    )
                    .await?;
                } else {
                    //send_icmpv6_echo_reply(self).await?;
                }
            }
        } else {
            // NOROUTE for non-exists chains
            send_icmpv6_error_reply(
                self,
                &dst,
                &src,
                inet::ICMP6_DST_UNREACH,
                inet::ICMP6_DST_UNREACH_NOROUTE,
                0,
            )
            .await?;
        }
        Ok(())
    }
}

pub async fn send_icmpv6_error_reply(
    tun: &mut TunHandler,
    src: &Ipv6Addr,
    dst: &Ipv6Addr,
    typ: u32,
    code: u32,
    data: u32,
) -> Result<()> {
    let len = max(
        tun.recv_size - size_of::<inet::icmp6_hdr>() - size_of::<inet::ip6_hdr>(),
        min(1000, tun.recv_size),
    );
    build_icmpv6_reply(&mut tun.buf, true, src, dst, typ, code, data, len)?;
    tun.send(
        0,
        size_of::<inet::ip6_hdr>() + size_of::<inet::icmp6_hdr>() + len,
    )
    .await?;
    Ok(())
}

pub fn build_icmpv6_reply(
    buf: &mut TunBuffer,
    prepend: bool,
    src: &Ipv6Addr,
    dst: &Ipv6Addr,
    typ: u32,
    code: u32,
    data: u32,
    len: usize,
) -> Result<()> {
    let offset = if prepend { 0 } else { super::ERROR_HEADER_SIZE } + size_of::<inet::ip6_hdr>();
    let icmp6 = buf.object::<inet::icmp6_hdr>(offset);
    icmp6.icmp6_type = typ as u8;
    icmp6.icmp6_code = code as u8;
    icmp6.icmp6_cksum = 0;
    unsafe {
        icmp6.icmp6_dataun.icmp6_un_data32[0] = data;
    }
    let checksum = unsafe {
        ip::calc_checksum(
            buf.object(offset),
            size_of::<inet::icmp6_hdr>() + len,
            ip::calc_ipv6_phdr_checksum(
                src,
                dst,
                (size_of::<inet::icmp6_hdr>() + len) as u32,
                inet::IPPROTO_ICMPV6 as u8,
            ),
        )
    };
    icmp6.icmp6_cksum = checksum;
    ip::build_ipv6_reply(
        buf,
        prepend,
        src,
        dst,
        size_of::<inet::icmp6_hdr>() + len,
        inet::IPPROTO_ICMPV6,
    )?;
    Ok(())
}
