use std::net::Ipv6Addr;

use serde::{Deserialize, Serialize};

use crate::config::get_config;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SubnetConfig {
    pub subnet: Ipv6Addr,
    #[serde(default = "default_subnet_len")]
    pub subnet_len: u8,
    #[serde(default = "default_hop_len")]
    pub hop_len: u8,
}

fn default_subnet_len() -> u8 {
    64
}

fn default_hop_len() -> u8 {
    16
}

impl SubnetConfig {
    pub fn host_addr(&self) -> Ipv6Addr {
        self.with_hop(
            Ipv6Addr::from(u128::from(self.subnet) | (u128::MAX >> self.subnet_len)),
            0,
        )
    }

    pub fn with_hop(&self, addr: Ipv6Addr, hop: u16) -> Ipv6Addr {
        Ipv6Addr::from(u128::from(addr) >> self.hop_len << self.hop_len | hop as u128)
    }

    pub fn contains(&self, addr: Ipv6Addr) -> bool {
        u128::from(addr) >> self.subnet_len == u128::from(self.subnet) >> self.subnet_len
    }

    pub fn parse(&self, addr: Ipv6Addr) -> (u128, u8) {
        let addr = u128::from(addr) << self.subnet_len >> self.subnet_len;
        let hop_len = u128::BITS - self.hop_len as u32;
        (addr >> self.hop_len, (addr << hop_len >> hop_len) as u8)
    }
}

pub fn try_parse(addr: Ipv6Addr) -> Option<(u128, u8)> {
    for subnet in get_config().subnet.iter() {
        if subnet.contains(addr) {
            return Some(subnet.parse(addr));
        }
    }
    None
}
