use std::{mem::size_of, net::Ipv6Addr};

use anyhow::{bail, Result};

use crate::{inet, resolver::resolve, subnet};

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
        index: u128,
        hop: u8,
    ) -> Result<()>;
}

impl Icmp6Handler for TunHandler {
    async fn handle_icmpv6(
        &mut self,
        src: Ipv6Addr,
        dst: Ipv6Addr,
        index: u128,
        hop: u8,
    ) -> Result<()> {
        if let Some(chain) = resolve(index).await {}
        send_icmpv6_error_reply(
            self,
            &dst,
            &src,
            inet::ICMP6_DST_UNREACH,
            inet::ICMP6_DST_UNREACH_NOROUTE,
            0,
        )
        .await?;
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
    let len = tun.recv_size - size_of::<inet::icmp6_hdr>() - size_of::<inet::ip6_hdr>();
    build_icmpv6_reply(&mut tun.buf, true, src, dst, typ, code, data, len)?;
    tun.send(0, tun.recv_size).await?;
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
    let ip_offset = if prepend { 0 } else { super::ERROR_HEADER_SIZE };
    let icmp6 = buf.object::<inet::icmp6_hdr>(ip_offset + size_of::<inet::ip6_hdr>());
    icmp6.icmp6_type = typ as u8;
    icmp6.icmp6_code = code as u8;
    icmp6.icmp6_cksum = 0;
    unsafe {
        icmp6.icmp6_dataun.icmp6_un_data32[0] = data;
    }
    let checksum = unsafe {
        ip::calc_checksum(
            buf.object(ip_offset + size_of::<inet::ip6_hdr>()),
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
