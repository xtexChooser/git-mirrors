use std::collections::HashMap;

use anyhow::{bail, Error, Result};
use reqwest::{Client, Response, Url};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone)]
pub struct APIClient {
    pub client: Client,
    pub data: String,
}

impl APIClient {
    pub fn from(url: Url) -> Result<APIClient> {
        let mut fragment = url
            .fragment()
            .ok_or_else(|| Error::msg("URL does not have a fragment"))?;
        if let Some(i) = fragment.find('&') {
            fragment = &fragment[0..i];
        }
        return Ok(APIClient {
            client: Client::builder().build()?,
            data: fragment.to_owned(),
        });
    }

    pub async fn request<U: reqwest::IntoUrl>(
        &self,
        url: U,
        data: HashMap<&str, &str>,
    ) -> Result<Response> {
        let mut body = String::new();
        for (k, v) in data.iter() {
            body.push_str(&format!(
                "&{}={}",
                urlencoding::encode(k),
                urlencoding::encode(v)
            ));
        }
        if !body.is_empty() {
            body.remove(0);
        }
        Ok(self.client.post(url).body(body).send().await?)
    }

    pub async fn get_highest_scores(&self) -> Result<HighestScoresResponse> {
        info!("getting highest scores");
        let resp = self
            .request(
                "https://tbot.xyz/api/getHighScores",
                [("data", self.data.as_str())].into(),
            )
            .await?;
        Ok(resp.json().await?)
    }

    pub async fn get_my_record(&self) -> Result<u32> {
        let scores = self.get_highest_scores().await?.scores;
        for score in scores.iter() {
            if score.current == Some(true) {
                return Ok(score.score);
            }
        }
        bail!("record with current=Some(true) not found")
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HighestScoresResponse {
    pub scores: Vec<HighestScore>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HighestScore {
    pub pos: u32,
    pub score: u32,
    pub name: String,
    pub current: Option<bool>,
}
