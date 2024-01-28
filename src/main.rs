use anyhow::Result;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::app::App;

pub mod app;
pub mod db;
pub mod linter;
pub mod page;
pub mod web;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
	dotenvy::dotenv()?;
	tracing::subscriber::set_global_default(
		FmtSubscriber::builder()
			.with_env_filter(
				EnvFilter::builder()
					.with_default_directive(LevelFilter::INFO.into())
					.with_env_var("SPOCK_LOG")
					.from_env()?,
			)
			.json()
			.finish(),
	)?;

	info!("Startup");
	App::init().await?;

	tokio::spawn(web::run_server());

	linter::run_linter().await;

	Ok(())
}
