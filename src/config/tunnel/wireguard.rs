use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct WireGuardConfig {
    pub prefix: String,
}
