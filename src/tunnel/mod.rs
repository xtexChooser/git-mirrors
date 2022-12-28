use self::wireguard::WireGuardConfig;

pub mod wireguard;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TunnelConfig {
    WireGuard(WireGuardConfig),
}
