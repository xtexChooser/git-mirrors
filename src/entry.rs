use anyhow::Result;
use simple_logger::SimpleLogger;

use crate::{config::init_config, etcd::init_etcd, tunnel, zone::init_zones};

#[tokio::main]
pub async fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    info!("peer42d version {}", env!("CARGO_PKG_VERSION"));
    init_config().await?;
    init_etcd().await?;
    init_zones().await?;
    tunnel::wireguard::delete_unknown_if().await?;
    // @TODO: watch for changes
    Ok(())
}
