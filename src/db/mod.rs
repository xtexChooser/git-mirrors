use std::{sync::Arc, time::Duration};

use anyhow::Result;
use sea_orm::{ConnectOptions, DatabaseConnection};
use sea_orm_migration::MigratorTrait;

use crate::app::App;

pub mod migration;
pub mod model;

pub use model::*;

pub struct DatabaseManager {
	pub conn: Arc<DatabaseConnection>,
}

impl DatabaseManager {
	pub async fn new() -> Result<Self> {
		let mut opts = ConnectOptions::from(std::env::var("SPOCK_DATABASE")?);
		opts.sqlx_logging(true)
			.sqlx_logging_level(tracing::log::LevelFilter::Trace)
			.sqlx_slow_statements_logging_settings(
				tracing::log::LevelFilter::Warn,
				Duration::from_millis(50),
			);
		let conn = Arc::new(sea_orm::Database::connect(opts).await?);
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
