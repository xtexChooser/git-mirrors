use std::{collections::BTreeMap, io::Read, path::PathBuf};

use anyhow::{bail, Error, Result};
use bytes::Buf;
use flate2::read::GzDecoder;
use tokio::fs;
use tracing::info;

const URL: &str =
    "https://unifoundry.com/pub/unifont/unifont-15.0.01/font-builds/unifont-15.0.01.hex.gz";

pub const GLYPH_HEIGHT: u16 = 16;

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

#[derive(Debug, PartialEq, Eq)]
pub enum Glyph {
    HalfWidth([u8; 16]),
    FullWidth([u16; 16]),
}

impl Glyph {
    pub fn new(str: &str) -> Result<Glyph> {
        match str.len() {
            32 => {
                let hexes: [String; 16] = Vec::from_iter(str.chars())
                    .chunks(2)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| Error::msg("bad glyph").context(str.to_string()))?;
                Ok(Glyph::HalfWidth(
                    hexes
                        .try_map(|hex| u8::from_str_radix(&hex, 16))?
                        .try_into()
                        .map_err(|_| Error::msg("bad glyph").context(str.to_string()))?,
                ))
            }
            64 => {
                let hexes: [String; 16] = Vec::from_iter(str.chars())
                    .chunks(4)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|_| Error::msg("bad glyph").context(str.to_string()))?;
                Ok(Glyph::FullWidth(
                    hexes
                        .try_map(|hex| u16::from_str_radix(&hex, 16))?
                        .try_into()
                        .map_err(|_| Error::msg("bad glyph").context(str.to_string()))?,
                ))
            }
            _ => bail!("unknown glyph len {}", str),
        }
    }

    pub fn width(&self) -> usize {
        match self {
            Glyph::HalfWidth(_) => 8,
            Glyph::FullWidth(_) => 16,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        match self {
            Glyph::HalfWidth(r) => y < 16 && x < 8 && r[y] & (0x80 >> x) != 0,
            Glyph::FullWidth(r) => y < 16 && x < 16 && r[y] & (0x8000 >> x) != 0,
        }
    }
}
