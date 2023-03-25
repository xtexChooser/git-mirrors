use std::net::Ipv6Addr;

use anyhow::{bail, Result};

use crate::{inet, subnet};

use super::{buf::TunBuffer, icmp6::Icmp6Handler, TunHandler};

pub trait IpHandler {
    async fn handle_ip(&mut self) -> Result<()>;
    async fn handle_ipv6(&mut self) -> Result<()>;
}

impl IpHandler for TunHandler {
    async fn handle_ip(&mut self) -> Result<()> {
        let ip = self.buf.read_object::<inet::ip>(0);
        match ip.ip_v() {
            4 => (),
            6 => self.handle_ipv6().await?,
            _ => bail!("unknown IP version: {}", ip.ip_v()),
        }
        Ok(())
    }

    async fn handle_ipv6(&mut self) -> Result<()> {
        let ip6 = self.buf.read_object::<inet::ip6_hdr>(0);
        let dst = parse_in6_addr(&ip6.ip6_dst);
        if let Some((subnet, index, hop)) = subnet::try_parse(dst) {
            let src = parse_in6_addr(&ip6.ip6_src);
            match unsafe { ip6.ip6_ctlun.ip6_un1.ip6_un1_nxt } as u32 {
                inet::IPPROTO_ICMPV6 => self.handle_icmpv6(src, dst, subnet, index, hop).await?,
                _ => (),
            }
        }
        Ok(())
    }
}

pub fn parse_in6_addr(addr: &inet::in6_addr) -> Ipv6Addr {
    let addr = unsafe { addr.__in6_u.__u6_addr32 };
    let addr = (u32::from_be(addr[0]) as u128) << 96
        | (u32::from_be(addr[1]) as u128) << 64
        | (u32::from_be(addr[2]) as u128) << 32
        | (u32::from_be(addr[3]) as u128);
    Ipv6Addr::from(addr)
}

pub fn to_in6_addr(addr: &Ipv6Addr) -> inet::in6_addr {
    let addr = u128::from(*addr);
    inet::in6_addr {
        __in6_u: inet::in6_addr__bindgen_ty_1 {
            __u6_addr32: [
                u32::to_be((addr >> 96) as u32),
                u32::to_be((addr >> 64) as u32),
                u32::to_be((addr >> 32) as u32),
                u32::to_be(addr as u32),
            ],
        },
    }
}

pub fn build_ipv6_reply(
    buf: &mut TunBuffer,
    prepend: bool,
    src: &Ipv6Addr,
    dst: &Ipv6Addr,
    len: usize,
    next_header: u32,
) -> Result<()> {
    let read_ip6 = buf.read_object::<inet::ip6_hdr>(0);
    let ip6 = buf.object::<inet::ip6_hdr>(if prepend { 0 } else { super::ERROR_HEADER_SIZE });
    unsafe {
        ip6.ip6_ctlun.ip6_un1.ip6_un1_flow =
            ((6 << 28) | (0 << 20) | u32::from_be(read_ip6.ip6_ctlun.ip6_un1.ip6_un1_flow)).to_be();
        ip6.ip6_ctlun.ip6_un1.ip6_un1_plen = (len as u16).to_be();
        ip6.ip6_ctlun.ip6_un1.ip6_un1_nxt = next_header as u8;
        ip6.ip6_ctlun.ip6_un1.ip6_un1_hlim = 0xff;
    }
    ip6.ip6_src = to_in6_addr(src);
    ip6.ip6_dst = to_in6_addr(dst);
    Ok(())
}

pub unsafe fn calc_checksum(data: *const u8, size: usize, ext_sum: u32) -> u16 {
    let mut checksum = ext_sum;
    let data16 = data as *const u16;

    for i in 0..(size / 2) {
        checksum += *data16.offset(i as isize) as u32;
    }
    if size % 1 == 1 {
        checksum += *data.offset(size as isize) as u32;
    }

    while (checksum >> 16) != 0 {
        checksum = (checksum & 0xffff) + (checksum >> 16);
    }
    !(checksum as u16)
}

pub fn calc_ipv6_phdr_checksum(src: &Ipv6Addr, dst: &Ipv6Addr, len: u32, nh: u8) -> u32 {
    let mut checksum = 0u32;

    let src = unsafe { to_in6_addr(src).__in6_u.__u6_addr16 };
    let dst = unsafe { to_in6_addr(dst).__in6_u.__u6_addr16 };
    for i in 0..8 {
        checksum += src[i] as u32;
        checksum += dst[i] as u32;
    }

    checksum += (len as u16).to_be() as u32;
    checksum += (len >> 16).to_be() as u32;
    checksum += (nh as u32) << 8;

    checksum
}
