use std::net::Ipv6Addr;

use anyhow::{bail, Result};

use crate::{inet, subnet};

use super::TunHandler;

pub trait IcmpHandler {
    async fn handle_icmpv6(&mut self, src: Ipv6Addr, index: u128, hop: u8) -> Result<()>;
}

impl IcmpHandler for TunHandler {
    async fn handle_icmpv6(&mut self, src: Ipv6Addr, index: u128, hop: u8) -> Result<()> {
        Ok(())
    }
}
