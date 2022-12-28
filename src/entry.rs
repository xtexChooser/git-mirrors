use anyhow::Result;
use simple_logger::SimpleLogger;

use crate::{config::init_config, etcd::init_etcd, zone::init_zones};

#[tokio::main]
pub async fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    info!("peer42d version {}", env!("CARGO_PKG_VERSION"));
    init_config()?;
    init_etcd().await?;
    init_zones().await?;
    // @TODO: watch for changes
    Ok(())
}
