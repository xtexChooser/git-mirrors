use anyhow::Result;
use mekbuda::tun::start_tun;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().json().finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    start_tun().await?;

    Ok(())
}
