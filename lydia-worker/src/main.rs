use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("lydia worker starting");

    Ok(())
}
