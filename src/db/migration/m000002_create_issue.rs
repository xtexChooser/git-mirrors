use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Issue::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(Issue::Id)
							.uuid()
							.not_null()
							.primary_key()
							.unique_key(),
					)
					.col(ColumnDef::new(Issue::Page).uuid().not_null())
					.col(ColumnDef::new(Issue::FoundAt).timestamp().not_null())
					.col(ColumnDef::new(Issue::IssueType).string().not_null())
					.col(ColumnDef::new(Issue::Details).json_binary().not_null())
					.to_owned(),
			)
			.await?;
		manager
			.create_index(
				Index::create()
					.name("issues_id")
					.table(Issue::Table)
					.if_not_exists()
					.col(Issue::Id)
					.unique()
					.index_type(IndexType::BTree)
					.to_owned(),
			)
			.await?;
		manager
			.create_index(
				Index::create()
					.name("issues_page")
					.table(Issue::Table)
					.if_not_exists()
					.col(Issue::Page)
					.to_owned(),
			)
			.await?;
		manager
			.create_index(
				Index::create()
					.name("issues_type")
					.table(Issue::Table)
					.if_not_exists()
					.col(Issue::IssueType)
					.index_type(IndexType::Hash)
					.to_owned(),
			)
			.await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_index(
				Index::drop()
					.table(Issue::Table)
					.name("issues_id")
					.to_owned(),
			)
			.await?;
		manager
			.drop_index(
				Index::drop()
					.table(Issue::Table)
					.name("issues_page")
					.to_owned(),
			)
			.await?;
		manager
			.drop_index(
				Index::drop()
					.table(Issue::Table)
					.name("issues_type")
					.to_owned(),
			)
			.await?;
		manager
			.drop_table(Table::drop().table(Issue::Table).to_owned())
			.await?;
		Ok(())
	}
}

#[allow(clippy::enum_variant_names)]
#[derive(DeriveIden)]
enum Issue {
	Table,
	Id,
	Page,
	FoundAt,
	IssueType,
	Details,
}
