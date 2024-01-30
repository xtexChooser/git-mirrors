use anyhow::Result;
use spock::{app::App, db, linter, page, rcsyncer, web};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

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
	tokio::spawn(page::run_page_list_syncer());
	tokio::spawn(rcsyncer::run_rc_syncer());
	tokio::spawn(db::run_sqlite_interval_optimizer());

	linter::run_linter().await;

	Ok(())
}
