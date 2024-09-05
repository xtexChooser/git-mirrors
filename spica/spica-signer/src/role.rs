use anyhow::Result;
use serde::{Deserialize, Serialize};
use spica_signer_common::sign::new_totp;
use totp_rs::TOTP;

use crate::{acl::ACLRule, config::get_config};

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub auth: String,
    #[serde(default)]
    pub prefer_hash: Option<String>,
    #[serde(default)]
    pub otp_required: Option<bool>,
    pub acl: Vec<ACLRule>,
}

impl Role {
    pub fn get(id: &str) -> Option<&'static Role> {
        get_config().role.iter().find(|r| r.id == id)
    }

    pub fn to_totp(&self) -> Result<TOTP> {
        new_totp(&self.auth)
    }
}
