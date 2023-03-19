use std::net::Ipv6Addr;

use anyhow::{bail, Result};

use crate::inet;

use super::TunHandler;

pub trait HandleIpExt {
    async fn handle_ip(&mut self) -> Result<()>;
    async fn handle_ipv6(&mut self) -> Result<()>;
}

impl HandleIpExt for TunHandler {
    async fn handle_ip(&mut self) -> Result<()> {
        let ip = self.buf.read::<inet::ip>(0);
        match ip.ip_v() {
            4 => bail!("unexpected IPv4 pkt"),
            6 => self.handle_ipv6().await?,
            _ => bail!("unknown IP version: {}", ip.ip_v()),
        }
        Ok(())
    }

    async fn handle_ipv6(&mut self) -> Result<()> {
        let ip6 = self.buf.read::<inet::ip6_hdr>(0);
        let src = parse_in6_addr(&ip6.ip6_src);
        let dst = parse_in6_addr(&ip6.ip6_dst);
        println!("recevied pkt {} {}", src, dst);
        Ok(())
    }
}

pub fn parse_in6_addr(addr: &inet::in6_addr) -> Ipv6Addr {
    let addr = unsafe { addr.__in6_u.__u6_addr32 };
    let addr = (u32::from_be(addr[0]) as u128) << 96
        | (u32::from_be(addr[1]) as u128) << 64
        | (u32::from_be(addr[2]) as u128) << 32
        | (u32::from_be(addr[3]) as u128) << 0;
    Ipv6Addr::from(addr)
}
