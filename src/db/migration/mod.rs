pub use sea_orm_migration::prelude::*;

mod m000001_create_wikipage;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![Box::new(m000001_create_wikipage::Migration)]
	}

	fn migration_table_name() -> DynIden {
		Alias::new("db_migrations").into_iden()
	}
}
