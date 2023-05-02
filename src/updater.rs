use std::env;

use anyhow::{anyhow, Result};
use tokio::sync::oneshot::Sender;

pub fn should_update() -> Result<bool> {
    if env::var("BUILD_CLEAN_NO_UPDATES").is_ok() {
        return Ok(false);
    }
    Ok(true)
}

pub async fn check_update(tx: Sender<Option<String>>) -> Result<()> {
    let resp = reqwest::Client::builder()
        .build()?
        .get(concat!(
            "https://crates.io/api/v1/crates/",
            env!("CARGO_PKG_NAME")
        ))
        .header(
            "User-Agent",
            format!(
                "{}/{} (build-clean@xtexx.ml)",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
        )
        .send()
        .await?
        .error_for_status()?
        .json::<serde_json::Value>()
        .await?;
    let version = resp
        .as_object()
        .ok_or(anyhow!("crates.io respond a json but not obj"))?["crate"]
        .as_object()
        .ok_or(anyhow!("$.crate is not a json obj"))?["newest_version"]
        .as_str()
        .ok_or(anyhow!("$.crate.newest_version is not a json str"))?
        .to_string();
    if version != env!("CARGO_PKG_VERSION") {
        let _ = tx.send(Some(version));
    } else {
        let _ = tx.send(None);
    }
    Ok(())
}
