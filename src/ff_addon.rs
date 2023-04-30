use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use thirtyfour::{extensions::addons::firefox::FirefoxTools, session::handle::SessionHandle};

pub fn find_firefox_addon() -> Result<PathBuf> {
    let dev_path = Path::new("ff_addon/addon.zip");
    if dev_path.exists() {
        return Ok(dev_path.to_owned());
    }
    let prod_path = Path::new("ff_addon.zip");
    if prod_path.exists() {
        return Ok(prod_path.to_owned());
    }
    bail!("firefox addon not found")
}

pub async fn install_firefox_addon(handle: SessionHandle, path: &str) -> Result<()> {
    FirefoxTools::new(handle)
        .install_addon(path, Some(true))
        .await?;
    Ok(())
}
