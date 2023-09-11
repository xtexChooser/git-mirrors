use anyhow::{Context, Error, Result};

pub mod linux;

pub fn parse_wg_key(key: &str) -> Result<[u8; 32]> {
    let result = <[u8; 32]>::try_from(base64::decode(key).context("decode base64 for WG key")?);
    match result {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::msg(format!(
            "convert slice for WG key {} to [u8; 32]",
            key
        ))),
    }
}
