pub use sea_orm_migration::prelude::*;

mod m000001_create_page;
mod m000002_create_issue;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
			Box::new(m000001_create_page::Migration),
			Box::new(m000002_create_issue::Migration),
		]
	}

	fn migration_table_name() -> DynIden {
		Alias::new("db_migrations").into_iden()
	}
}
