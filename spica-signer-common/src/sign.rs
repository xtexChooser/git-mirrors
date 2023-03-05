use anyhow::Result;
use totp_rs::{Rfc6238, TOTP};

pub fn new_totp(secret: &str) -> Result<TOTP> {
    Ok(TOTP::from_rfc6238(Rfc6238::with_defaults(
        secret.as_bytes().to_vec(),
    )?)?)
}
