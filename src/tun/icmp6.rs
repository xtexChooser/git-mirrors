use std::{
    cmp::{max, min},
    mem::size_of,
    net::Ipv6Addr,
};

use anyhow::Result;

use crate::inet;

use super::{
    buf::TunBuffer,
    ip::{self},
    TunHandler,
};

pub trait Icmp6Handler {
    async fn handle_icmpv6(&mut self, src: Ipv6Addr, dst: Ipv6Addr) -> Result<()>;
}

impl Icmp6Handler for TunHandler {
    async fn handle_icmpv6(&mut self, src: Ipv6Addr, dst: Ipv6Addr) -> Result<()> {
        let icmp6 = self
            .buf
            .read_object::<inet::icmp6_hdr>(size_of::<inet::ip6_hdr>());
        if icmp6.icmp6_type == inet::ICMP6_ECHO_REQUEST as u8 && icmp6.icmp6_code == 0 {
            send_echo_reply(self, &dst, &src).await?;
        }
        Ok(())
    }
}

pub async fn send_error_reply(
    tun: &mut TunHandler,
    src: &Ipv6Addr,
    dst: &Ipv6Addr,
    typ: u32,
    code: u32,
) -> Result<()> {
    let len = max(
        tun.recv_size - size_of::<inet::icmp6_hdr>() - size_of::<inet::ip6_hdr>(),
        min(1000, tun.recv_size),
    );
    build_reply(&mut tun.buf, true, src, dst, typ, code, len)?;
    tun.send(
        0,
        size_of::<inet::ip6_hdr>() + size_of::<inet::icmp6_hdr>() + len,
    )
    .await?;
    Ok(())
}

pub fn build_reply(
    buf: &mut TunBuffer,
    prepend: bool,
    src: &Ipv6Addr,
    dst: &Ipv6Addr,
    typ: u32,
    code: u32,
    len: usize,
) -> Result<()> {
    let offset = if prepend { 0 } else { super::ERROR_HEADER_SIZE } + size_of::<inet::ip6_hdr>();
    let icmp6 = buf.object::<inet::icmp6_hdr>(offset);
    icmp6.icmp6_type = typ as u8;
    icmp6.icmp6_code = code as u8;
    icmp6.icmp6_cksum = 0;
    unsafe {
        icmp6.icmp6_dataun.icmp6_un_data32[0] = 0;
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

pub async fn send_echo_reply(tun: &mut TunHandler, src: &Ipv6Addr, dst: &Ipv6Addr) -> Result<()> {
    let ip6 = tun.buf.read_object::<inet::ip6_hdr>(0);
    let icmp6 = tun
        .buf
        .read_object::<inet::icmp6_hdr>(size_of::<inet::ip6_hdr>());

    ip6.ip6_src = ip::to_in6_addr(src);
    ip6.ip6_dst = ip::to_in6_addr(dst);
    ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim = ip::REPLY_TTL;

    icmp6.icmp6_type = inet::ICMP6_ECHO_REPLY as u8;
    ip::diff_checksum(
        &mut icmp6.icmp6_cksum,
        inet::ICMP6_ECHO_REQUEST as u16,
        inet::ICMP6_ECHO_REPLY as u16,
    );

    tun.send(super::ERROR_HEADER_SIZE, tun.recv_size).await?;

    Ok(())
}
