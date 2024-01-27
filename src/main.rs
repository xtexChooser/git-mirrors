use anyhow::Result;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::app::App;

pub mod app;
pub mod linter;
pub mod web;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
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

	info!("spock.start");
	let app = App::new();

	tokio::spawn(web::run_server(app.to_owned()));

	linter::run_linter(app.to_owned()).await;

	Ok(())
}
