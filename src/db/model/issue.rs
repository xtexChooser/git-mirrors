use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "issue")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false, unique, indexed)]
	pub id: Uuid,
	#[sea_orm(column_type = "Uuid", indexed)]
	pub page: Uuid,
	#[sea_orm(column_type = "String(None)", indexed)]
	pub issue_type: String,
	#[sea_orm(column_type = "JsonBinary")]
	pub details: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::page::Entity",
		from = "Column::Page",
		to = "super::page::Column::Id"
	)]
	Page,
}

impl Related<super::page::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Page.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
