use anyhow::Result;
use chrono::{Months, Utc};
use log::info;
use mwbot::{Bot, SaveOptions};
use wiki_bot::utils::{get_bot, init};

#[tokio::main]
async fn main() -> Result<()> {
    init().await?;
    let bot = get_bot().await?;
    archive(&bot, "DN42对等请求").await?;
    archive(&bot, "Wiki:管理员告示板").await?;
    Ok(())
}

async fn archive(bot: &Bot, target: &str) -> Result<()> {
    let to_page = format!(
        "{}/存档/{}",
        target,
        (Utc::now() - Months::new(1)).format("%Y-%m")
    );
    info!("archiving {}, to {}", target, to_page);
    let move_resp: serde_json::Value = bot
        .api()
        .post_with_token(
            "csrf",
            &[
                ("action", "move"),
                ("from", target),
                ("to", to_page.as_str()),
                ("reason", "Auto Archive"),
                ("noredirect", "true"),
                ("watchlist", "nochange"),
            ],
        )
        .await?;
    info!("moved {}: {}", target, move_resp["move"]);
    let page = bot.page(target)?;
    assert!(!page.exists().await?);
    let (_, save_resp) = page
        .save(
            format!("{{{{:{}/header}}}}", target),
            &SaveOptions::summary("Re-create after auto archive"),
        )
        .await?;
    info!("re-created {}: {:?}", target, save_resp);
    Ok(())
}
