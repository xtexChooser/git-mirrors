#![feature(absolute_path)]

use std::{
    cmp::{max, min},
    env, path,
    time::Duration,
};

use anyhow::Result;
use lumberjack::{
    api::APIClient,
    ff_addon::{find_firefox_addon, install_firefox_addon},
    game::{GameInterface, PlayerSide},
    gui::run_gui,
};
use reqwest::Url;
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::time::{self, Instant};
use tracing::{info, metadata::LevelFilter, trace};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    info!(concat!(
        env!("CARGO_PKG_NAME"),
        " ",
        env!("CARGO_PKG_VERSION")
    ),);
    trace!(
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        license = env!("CARGO_PKG_LICENSE"),
        rust_version = env!("CARGO_PKG_RUST_VERSION"),
        "build info"
    );

    default_var("LUMBERJACK_GAME_URL", "");
    default_var("LUMBERJACK_WEBDRIVER_URL", "http://127.0.0.1:4444");
    default_var(
        "LUMBERJACK_FIREFOX_ADDON",
        &path::absolute(find_firefox_addon()?)
            .map_err(anyhow::Error::from)?
            .to_string_lossy()
            .to_string(),
    );
    default_var("LUMBERJACK_TARGET_SCORES", "100");
    default_var("LUMBERJACK_STEP_SCORES", "50");

    if env::var("SHOW_GUI")
        .or_else(|_| env::var("LUMBERJACK_GUI"))
        .map(|s| s == "true")
        .unwrap_or(true)
    {
        run_gui().await?;
    }

    run().await?;

    Ok(())
}

fn default_var(k: &str, v: &str) {
    if let Err(_) = env::var(k) {
        env::set_var(k, v);
    }
}

pub async fn run() -> Result<()> {
    info!("run solver");

    let game_url = env::var("LUMBERJACK_GAME_URL")?;
    let driver_url = env::var("LUMBERJACK_WEBDRIVER_URL")?;
    let ff_addon = env::var("LUMBERJACK_FIREFOX_ADDON")?;
    let target_record: u32 = env::var("LUMBERJACK_TARGET_SCORES")?.parse()?;
    let step_score: u32 = env::var("LUMBERJACK_STEP_SCORES")?.parse()?;
    info!(
        game_url,
        driver_url, ff_addon, target_record, "retrieved configuration"
    );

    let driver = WebDriver::new(&driver_url, DesiredCapabilities::firefox()).await?;
    let api = APIClient::from(Url::parse(&game_url)?)?;
    info!(data = api.data, "created API client");
    let mut current_record = api.get_my_record().await?;
    info!(current_record, "got current record");

    driver.minimize_window().await?;
    install_firefox_addon(driver.handle.clone(), &ff_addon).await?;

    driver.goto(game_url).await?;
    assert_eq!(driver.title().await?, "LumberJack");
    info!("opened target game url");

    info!("wait game to ready");
    let t0 = Instant::now();
    while !driver.is_ready().await? {
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    info!(time = t0.elapsed().as_millis(), "game is ready");

    let t0 = Instant::now();
    while current_record < target_record {
        let t1 = Instant::now();
        let now_target_record = min(current_record + step_score, target_record);
        info!(now_target_record, "starting new game");

        assert!(driver.is_finished().await?);
        driver.start().await?;
        while driver.is_finished().await? {}

        let mut score = 0u32;
        let mut level = 0u32;
        let mut max_respond_time = 250u32;
        while !driver.is_finished().await? {
            if score >= now_target_record {
                time::sleep(Duration::from_millis(100)).await;
                continue;
            }
            let sides = driver.pull_incoming().await?;
            for side in sides.into_iter() {
                match side {
                    PlayerSide::Left => driver.click_left().await?,
                    PlayerSide::Right => driver.click_right().await?,
                }
                score += 1;
                if (score % 20) == 0 {
                    level += 1;
                    max_respond_time = (max_respond_time as f32 * 0.9) as u32;
                    info!(score, level, max_respond_time, "level up");
                }
                if score >= now_target_record {
                    break;
                }
                time::sleep(Duration::from_millis(
                    max(min(max_respond_time as i32 - 20, 50), 5) as u64,
                ))
                .await;
            }
        }
        current_record = score;

        info!(time = t1.elapsed().as_millis(), "game is finished");
    }
    info!(time = t0.elapsed().as_millis(), "reached target score");

    Ok(())
}
