use crate::tunnel::TunnelConfig;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PeerConfig {
    pub tun: TunnelConfig,
}
