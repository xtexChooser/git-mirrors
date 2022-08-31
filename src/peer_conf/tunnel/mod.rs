use serde::Deserialize;

use self::wireguard::WireGuardConfig;

pub mod wireguard;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash, Default)]
pub struct TunnelConfig {
    #[serde(default)]
    pub wireguard: Vec<WireGuardConfig>,
}
