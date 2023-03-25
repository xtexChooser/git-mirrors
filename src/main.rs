use anyhow::Result;
use mekbuda::{resolver, tun::start_tun};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tokio::spawn(resolver::cache::gc_worker());
    start_tun().await?;

    Ok(())
}
