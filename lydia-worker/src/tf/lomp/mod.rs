use std::{collections::HashMap, fs, sync::Arc};

use anyhow::Result;
use askama::Template;
use chrono::{DateTime, Duration, Utc};
use mwbot::SaveOptions;
use serde::{Deserialize, Serialize};
use tokio::{task::JoinHandle, time::sleep};
use tracing::{info, instrument, warn};

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

#[instrument(skip(bot))]
async fn do_lomp_maintaince(bot: Arc<Bot>) -> Result<()> {
    info!("start LOMP maintaince");
    let mwbot = bot.zhwp.clone().unwrap();

    let mps = mpc::fetch_numbered_mps(&bot).await?;

    // pull all subpages
    info!("scanning exists subpages");
    let (tx, mut rx) = tokio::sync::mpsc::channel(50);
    {
        let mwbot = mwbot.clone();
        tokio::spawn(async move {
            let params = HashMap::from([
                ("action".to_string(), "query".to_string()),
                ("generator".to_string(), "allpages".to_string()),
                ("gapprefix".to_string(), "小行星列表/".to_string()),
                ("gapnamespace".to_string(), "0".to_string()),
                ("gaplimit".to_string(), "max".to_string()),
            ]);
            let mut continue_: HashMap<String, String> = HashMap::new();
            loop {
                let mut merged_params: HashMap<String, String> = HashMap::new();
                merged_params.extend(params.clone());
                merged_params.extend(continue_.clone());
                let resp = mwbot
                    .api()
                    .get::<_, serde_json::Value>(merged_params)
                    .await
                    .unwrap();
                continue_ =
                    serde_json::from_value(resp["continue"].clone()).unwrap_or(HashMap::new());
                for page in resp["query"]["pages"].as_array().unwrap() {
                    tx.send(page["title"].as_str().unwrap().to_string())
                        .await
                        .unwrap();
                }
                if continue_.is_empty() {
                    break;
                }
            }
        });
    }
    let mut subpages = vec![];
    while let Some(page) = rx.recv().await {
        let page = &page[16..];
        if let Some((start, end)) = page.split_once('-') {
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            let range = start..=end;
            let len = end - start;
            if len != 99 && len != 999 {
                warn!(page, start, end, "strange page range (len)");
            }
            if (len == 99 && start / 100 * 100 + 1 != start)
                || (len == 999 && start / 1000 * 1000 + 1 != start)
            {
                warn!(page, start, end, "strange page range (start)");
            }
            subpages.push(range);
        }
    }

    // check hundreds pages
    info!("checking hundreds pages");
    let mut hundreds_pages = mps
        .keys()
        .map(|k| k / 100)
        .map(|k| (k * 100 + 1)..=((k + 1) * 100))
        .collect::<Vec<_>>();
    hundreds_pages.dedup();
    hundreds_pages.retain(|r| !subpages.contains(r));

    info!(count = hundreds_pages.len(), "missing hundreds pages");

    // create hundreds pages
    for range in &hundreds_pages {
        let mut mps = mps
            .iter()
            .filter(|(n, _)| range.contains(n))
            .map(|(k, v)| (*k, v))
            .collect::<Vec<_>>();
        mps.sort_by_key(|(n, _)| *n);
        assert_ne!(mps.len(), 0);
        let page = format!("小行星列表/{}-{}", range.start(), range.end());
        let mut content = String::new();

        // build table
        content.push_str("<noinclude>{{小行星列表/Header2}}</noinclude>\n");
        for (num, mp) in mps {
            content.push_str("{{小行星列表/Item");
            content.push_str(&format!("|{}", num));
            content.push_str(&format!("|pname={}", num));
            if let Some(name) = &mp.name {
                content.push_str(&format!(" {}", name));
            }
            if let Some(desig) = &mp.principal_desig {
                content.push_str(&format!("|desig={}", desig));
            }
            // @TODO: meaning ref
            // @TODO: discovery rule
            content.push_str(&format!("|date={}", mp.discovery_date));
            content.push_str(&format!("|site={}", mp.discovery_site)); // @TODO: translations
            content.push_str(&format!("|discoverer={}", mp.discoverers)); // @TODO: translations
            content.push_str("}}\n");
        }
        content.push_str("|}\n");

        // save
        info!(page, content, "creating LOMP hundreds page");
        if !bot.is_dev() {
            // @TODO: not approved
            /*
            let mwpage = mwbot.page(&page)?;
            assert!(!mwpage.exists().await?);
            mwpage
                .save(
                    content,
                    &SaveOptions::summary("[lydia bot] 从MPC数据库生成小行星列表百位页面"),
                )
                .await?;
            */
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize, Template)]
#[template(path = "lomp_report.html")]
pub struct LOMPReport {
    pub time: DateTime<Utc>,
}
