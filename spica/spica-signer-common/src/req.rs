use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
pub struct CSR {
    pub name: HashMap<String, String>,
    #[serde(default)]
    pub expiry: Option<Duration>,
    #[serde(default)]
    pub not_before: Option<SystemTime>,
    #[serde(default)]
    pub serial: Option<String>,
    #[serde(default)]
    pub hosts: Vec<String>,
    #[serde(default)]
    pub public_key_pem: Option<String>,
    #[serde(default)]
    pub extra_ossl_opts: HashMap<String, String>,
}
