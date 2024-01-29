use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(RcSyncer::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(RcSyncer::Id)
							.uuid()
							.not_null()
							.primary_key()
							.unique_key(),
					)
					.col(
						ColumnDef::new(RcSyncer::LastSyncedAt)
							.timestamp()
							.not_null(),
					)
					.col(
						ColumnDef::new(RcSyncer::LastRcId)
							.unsigned()
							.default(0)
							.not_null(),
					)
					.to_owned(),
			)
			.await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(RcSyncer::Table).to_owned())
			.await?;
		Ok(())
	}
}

#[derive(DeriveIden)]
enum RcSyncer {
	Table,
	Id,
	LastSyncedAt,
	LastRcId,
}
