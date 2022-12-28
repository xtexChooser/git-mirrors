use std::net::IpAddr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WireGuardConfig {
    endpoint: IpAddr,
    endpoint_port: u16,
}
