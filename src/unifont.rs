use std::{collections::BTreeMap, io::Read, path::PathBuf};

use anyhow::Result;
use bytes::Buf;
use flate2::read::GzDecoder;
use tokio::fs;
use tracing::info;

const URL: &str =
    "https://unifoundry.com/pub/unifont/unifont-15.0.01/font-builds/unifont-15.0.01.hex.gz";

pub const GLYPH_HEIGHT: u32 = 16;

pub fn get_path() -> Result<PathBuf> {
    Ok(std::env::var("UNIFONT_FILE")
        .unwrap_or("unifont.hex".to_owned())
        .into())
}

pub async fn download() -> Result<()> {
    let path = get_path()?;
    if path.exists() {
        info!(
            path = path.to_string_lossy().to_string(),
            "unifont already exists"
        );
    } else {
        info!(
            path = path.to_string_lossy().to_string(),
            URL, "downloading unifont",
        );
        let resp = reqwest::get(URL)
            .await?
            .error_for_status()?
            .bytes()
            .await?
            .reader();
        let mut dec = GzDecoder::new(resp);
        let mut data = Vec::<u8>::new();
        dec.read_to_end(&mut data)?;
        fs::write(path, data).await?;
        info!("unifont downloaded");
    }
    Ok(())
}

pub async fn read() -> Result<BTreeMap<u32, String>> {
    let mut map = BTreeMap::new();
    info!("loading unifont");
    for line in fs::read_to_string(get_path()?).await?.lines() {
        if let Some(pos) = line.find(':') {
            let (code, mut glyph) = line.split_at(pos);
            glyph = &glyph[1..];

            let code = u32::from_str_radix(code, 16)?;
            map.insert(code, glyph.to_string());
        }
    }
    info!(glyph_count = map.len(), "unifont loaded");
    Ok(map)
}

pub static mut FONT: BTreeMap<u32, String> = BTreeMap::new();
