use anyhow::{bail, Result};

use crate::inet;

use super::TunHandler;

pub trait HandleIpExt {
    async fn handle_ip(&mut self) -> Result<()>;
}

impl HandleIpExt for TunHandler {
    async fn handle_ip(&mut self) -> Result<()> {
        let ip = self.buf.read::<inet::ip>(0);
        match ip.ip_v() {
            4 => bail!("unexpected IPv4 pkt"),
            _ => bail!("unknown IP version: {}", ip.ip_v()),
        }
        Ok(())
    }
}
