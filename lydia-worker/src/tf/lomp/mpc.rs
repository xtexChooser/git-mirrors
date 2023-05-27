use std::{collections::HashMap, io::Read};

use anyhow::Result;
use bytes::Buf;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use crate::Bot;

#[instrument(skip(bot))]
pub async fn fetch_numbered_mps(bot: &Bot) -> Result<HashMap<u64, NumberedMinorPlanet>> {
    info!("downloading numbered mps data from MPC");
    let bytes = bot
        .http
        .get("https://minorplanetcenter.net/Extended_Files/numberedmps.json.gz")
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let mut gz = GzDecoder::new(bytes.reader());
    let mut buf = String::new();
    gz.read_to_string(&mut buf)?;
    Ok(
        serde_json::from_str::<HashMap<String, NumberedMinorPlanet>>(&buf)?
            .into_iter()
            .map(|(k, v)| (k.parse().unwrap(), v))
            .collect(),
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumberedMinorPlanet {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Discovery_date")]
    pub discovery_date: String,
    #[serde(rename = "Discovery_rule")]
    pub discovery_rule: String,
    #[serde(rename = "Discovery_site")]
    pub discovery_site: String,
    #[serde(rename = "Discoverers")]
    pub discoverers: String,
    #[serde(rename = "Principal_desig")]
    pub principal_desig: Option<String>,
    #[serde(rename = "Ref")]
    pub meaning_ref: Option<String>,
}
