use std::{
    mem::{size_of, swap},
    net::Ipv6Addr,
};

use anyhow::Result;

use crate::inet;

use super::{
    ip::{self},
    TunHandler,
};

pub trait Tcp6Handler {
    async fn handle_tcp6(&mut self, src: Ipv6Addr, dst: Ipv6Addr) -> Result<()>;
}

impl Tcp6Handler for TunHandler {
    async fn handle_tcp6(&mut self, src: Ipv6Addr, dst: Ipv6Addr) -> Result<()> {
        let tcp = self
            .buf
            .read_object::<inet::tcphdr>(size_of::<inet::ip6_hdr>());
        if unsafe { tcp.__bindgen_anon_1.__bindgen_anon_1.th_flags } == inet::TH_SYN as u8 {
            send_rst_reply(self, &dst, &src).await?;
        }
        Ok(())
    }
}

pub async fn send_rst_reply(tun: &mut TunHandler, src: &Ipv6Addr, dst: &Ipv6Addr) -> Result<()> {
    let ip6 = tun.buf.read_object::<inet::ip6_hdr>(0);
    let tcp = tun
        .buf
        .read_object::<inet::tcphdr>(size_of::<inet::ip6_hdr>());
    let th = unsafe { &mut tcp.__bindgen_anon_1.__bindgen_anon_1 };

    // put addr
    ip6.ip6_src = ip::to_in6_addr(src);
    ip6.ip6_dst = ip::to_in6_addr(dst);
    ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim = ip::REPLY_TTL;

    // swap port
    swap(&mut th.th_dport, &mut th.th_sport);

    // TCP seq & ack
    th.th_ack = (u32::from_be(th.th_seq) + 1).to_be();
    th.th_seq = 0;

    // mark RST & ACK
    th.th_flags = (inet::TH_RST | inet::TH_ACK) as u8;

    // clear win
    th.th_win = 0;

    // clear opts
    th.set_th_off(size_of::<inet::tcphdr>() as u8 / 4);
    ip6.ip6_ctlun.ip6_un1.ip6_un1_plen = (size_of::<inet::tcphdr>() as u16).to_be();

    // update sum
    th.th_sum = 0;
    th.th_sum = unsafe {
        ip::calc_checksum(
            tun.buf.read_object(size_of::<inet::ip6_hdr>()),
            size_of::<inet::tcphdr>(),
            ip::calc_ipv6_phdr_checksum(
                src,
                dst,
                size_of::<inet::tcphdr>() as u32,
                inet::IPPROTO_TCP as u8,
            ),
        )
    };

    tun.send(
        super::ERROR_HEADER_SIZE,
        size_of::<inet::ip6_hdr>() + size_of::<inet::tcphdr>(),
    )
    .await?;
    Ok(())
}
