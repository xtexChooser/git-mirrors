use std::collections::BTreeMap;

use anyhow::{bail, Result};
use mwbot::SaveOptions;
use tracing::info;

use crate::{discord::send_discord, BOT};

pub async fn load_translation_page(
    bot: &mwbot::Bot,
    page: &str,
    required_keys: &Vec<&str>,
) -> Result<BTreeMap<String, String>> {
    info!(page, "fetching translations");
    let wt = bot.page(page)?.wikitext().await?;
    let map = wt
        .lines()
        .filter(|s| s.starts_with("* "))
        .filter_map(|s| s.split_once("=>"))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .filter(|(_, v)| v != "MISSING")
        .collect::<BTreeMap<_, _>>();

    let mut missing_keys = vec![];
    for k in required_keys {
        let k = k.to_string();
        if !map.contains_key(&k) {
            missing_keys.push(k);
        }
    }
    if !missing_keys.is_empty() {
        // missing keys
        let mwpage = bot.page(page)?;
        let mut wt = wt;
        if !wt.ends_with('\n') {
            wt.push('\n');
        }
        missing_keys.dedup();
        missing_keys.sort();
        for k in &missing_keys {
            wt.push_str(&format!("* {} => MISSING\n", k));
        }
        mwpage.save(wt, &SaveOptions::summary("summary")).await?;
        info!(page, "appended missing translations");
        let url = bot.page(page)?.url().await?.to_string();
        let lbot = BOT.read().clone().unwrap();
        send_discord(&lbot, |msg| {
            msg.embed(|e| e.title("缺失翻译").url(&url).description("补全已提交"))
        })
        .await?;
        bail!(
            "the following keys are missing in {}: {:?}",
            page,
            missing_keys
        )
    }

    Ok(map)
}
