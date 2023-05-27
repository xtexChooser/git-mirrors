use anyhow::Result;
use env::Env;
use secrets::Secrets;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::{fmt::writer::MakeWriterExt, FmtSubscriber};

pub mod env;
pub mod secrets;
pub mod tf;

#[derive(Serialize, Deserialize)]
pub struct Bot {
    pub env: Env,
    pub secrets: Secrets,
}

impl Bot {
    pub async fn new() -> Result<Bot> {
        Ok(Bot {
            env: env::detect_env()?,
            secrets: Secrets::new()?,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // init logging
    let file_appender = tracing_appender::rolling::daily("../logs", "worker.log");
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_writer(std::io::stdout.and(non_blocking_file))
        .json()
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // construct bot
    info!("lydia-worker started");
    let mut bot = Bot::new().await?;

    // run bot
    info!("run bot");
    bot.run().await
}
