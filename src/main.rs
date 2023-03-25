use anyhow::Result;
use mekbuda::{dns::start_dns, resolver, tun::start_tun};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tokio::spawn(resolver::cache::gc_worker());
    tokio::spawn(start_dns());
    start_tun().await?;

    Ok(())
}
