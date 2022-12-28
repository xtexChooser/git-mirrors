use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct WireGuardConfig {
    endpoint: Option<SocketAddr>,
    private_key: String,
    listen_port: Option<u16>,
    fw_mark: Option<u32>,
    remote_public_key: String,
    preshared_key: Option<String>,
}
