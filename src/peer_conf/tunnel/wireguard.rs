use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    pub listen_port: Option<u16>,
    pub private_key: String,
    pub preshared_key: Option<String>,
    pub endpoint: Option<String>,
    pub allowed_ips: Vec<String>,
    pub keep_alive: Option<u16>,
    pub fwmark: Option<u32>,
}
