use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    #[serde(default)]
    pub backend: WGBackend,
    pub prefix: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum WGBackend {
    LINUX,
    XPLATFORM,
}

impl Default for WGBackend {
    fn default() -> Self {
        Self::LINUX
    }
}
