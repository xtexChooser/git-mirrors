use anyhow::Result;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
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

	info!("Init");

	Ok(())
}
