use serde::Deserialize;

use self::wireguard::WireGuardConfig;

pub mod wireguard;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TunnelConfig {
    pub wireguard: Option<WireGuardConfig>,
}
