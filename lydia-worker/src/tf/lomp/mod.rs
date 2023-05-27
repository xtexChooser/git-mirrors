use std::sync::Arc;

use anyhow::Result;
use askama::Template;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use tokio::{task::JoinHandle, time::sleep};
use tracing::info;

use crate::Bot;

pub mod mpc;

pub const JSON_REPORT: &str = "tf/zhwiki/lomp.json";
pub const HTML_REPORT: &str = "tf/zhwiki/lomp.html";

pub fn start_lomp_worker(bot: Arc<Bot>) -> Result<JoinHandle<()>> {
    Ok(tokio::spawn(async move {
        if bot.is_dev() {
            do_lomp_maintaince(bot).await.unwrap();
        } else {
            loop {
                sleep(Duration::days(1).to_std().unwrap()).await;

                let bot = bot.clone();
                tokio::spawn(async move { do_lomp_maintaince(bot) });
            }
        }
    }))
}

async fn do_lomp_maintaince(bot: Arc<Bot>) -> Result<()> {
    info!("start LOMP maintaince");

    let mps = mpc::fetch_numbered_mps(&bot).await?;

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize, Template)]
#[template(path = "lomp_report.html")]
pub struct LOMPReport {
    pub time: DateTime<Utc>,
}
