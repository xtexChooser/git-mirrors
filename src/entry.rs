use anyhow::Result;
use futures::future::join_all;
use simple_logger::SimpleLogger;

use crate::{
    config::init_config,
    etcd::init_etcd,
    tunnel,
    zone::{init_zones, watch_zones},
};

#[tokio::main]
pub async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();
    //std::env::set_var("RUST_BACKTRACE", "1");
    info!("version {}", env!("CARGO_PKG_VERSION"));
    (async || -> Result<()> {
        init_config().await?;
        init_etcd().await?;
        init_zones().await?;
        tunnel::wireguard::delete_unknown_if().await?;
        join_all(watch_zones().await?).await;
        Ok(())
    })()
    .await
    .unwrap();
}
