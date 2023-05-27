use anyhow::Result;
use env::Env;
use secrets::Secrets;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod env;
pub mod secrets;
pub mod tf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub env: Env,
    pub secrets: Secrets,
}

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .json()
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("lydia worker starting");
    env::detect_env()?;

    Ok(())
}
