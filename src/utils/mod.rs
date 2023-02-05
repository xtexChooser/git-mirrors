use anyhow::Result;
use mwbot::Bot;

pub async fn init() -> Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::Level::Info.to_level_filter())
        .env()
        .init()
        .unwrap();
    Ok(())
}

pub async fn get_bot() -> Result<Bot> {
    Ok(Bot::from_default_config().await?)
}
