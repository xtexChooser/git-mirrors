use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use server::IdServer;

mod database;
mod server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[arg(long, default_value = "false")]
    update: bool,
}

#[actix_web::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt().json().init();

    let server =
        IdServer::new(&args.config.unwrap_or(PathBuf::from("odino.toml")))
            .await?;

    if args.update {
        database::schema::update(&server.database)
            .await
            .context("Run database schema update")?;
        return Ok(());
    }

    server.run().await
}
