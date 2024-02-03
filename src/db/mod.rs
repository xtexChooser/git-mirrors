use std::{sync::Arc, time::Duration};

use anyhow::Result;
use sea_orm::{ConnectOptions, ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;
use tracing::{error, info_span, Instrument};

use crate::{app::App, config, site};

pub mod migration;
pub mod model;

pub use model::*;

const SQLITE_PRAGMAS: phf::OrderedSet<&str> = phf::phf_ordered_set![
	"pragma journal_mode = WAL;",
	"pragma synchronous = normal;",
	"pragma temp_store = memory;",
	"pragma mmap_size = 4300000000;"
];

config!(DATABASE, str, required);
config!(DATABASE_SQLITE_INIT_PRAGMAS, parse bool, true);
config!(DATABASE_SQLITE_INTERVAL_OPTIMIZE, parse bool, true);

pub struct DatabaseManager {
	pub conn: Arc<DatabaseConnection>,
}

impl DatabaseManager {
	pub async fn new() -> Result<Self> {
		let mut opts = ConnectOptions::from(*CONFIG_DATABASE);
		opts.sqlx_logging(true)
			.sqlx_logging_level(tracing::log::LevelFilter::Trace)
			.sqlx_slow_statements_logging_settings(
				tracing::log::LevelFilter::Warn,
				Duration::from_millis(500),
			);
		let conn = Arc::new(sea_orm::Database::connect(opts).await?);
		if CONFIG_DATABASE.starts_with("sqlite") && *CONFIG_DATABASE_SQLITE_INIT_PRAGMAS {
			for stmt in &SQLITE_PRAGMAS {
				conn.execute(Statement::from_string(DbBackend::Sqlite, *stmt))
					.await?;
			}
		}

		migration::Migrator::up(conn.as_ref(), None).await?;

		Ok(Self { conn })
	}

	pub fn get() -> Arc<Self> {
		App::get().db.to_owned()
	}
}

pub fn get() -> Arc<DatabaseConnection> {
	DatabaseManager::get().conn.clone()
}

pub async fn run_sqlite_optimize() -> Result<()> {
	get()
		.execute(Statement::from_string(
			DbBackend::Sqlite,
			"pragma optimize;",
		))
		.await?;
	Ok(())
}

pub async fn run_sqlite_interval_optimizer() {
	if !*CONFIG_DATABASE_SQLITE_INTERVAL_OPTIMIZE {
		return;
	}

	tokio::time::sleep(std::time::Duration::from_secs(300)).await;

	loop {
		tokio::time::sleep(std::time::Duration::from_secs(
			site::SQLITE_INTERVAL_OPTIMIZE_PEROID,
		))
		.await;
		if let Err(error) = run_sqlite_optimize()
			.instrument(info_span!("sqlite_interval_optimize"))
			.await
		{
			error!(%error, "failed to run sqlite optimize");
		}
	}
}
