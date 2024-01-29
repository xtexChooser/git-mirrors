use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "issue")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub page: Uuid,
	pub issue_type: String,
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
