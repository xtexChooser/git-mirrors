use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, DeriveEntityModel)]
#[sea_orm(table_name = "page")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false, unique, indexed)]
	pub id: Uuid,
	#[sea_orm(column_type = "String(Some(8))")]
	pub lang: String,
	#[sea_orm(column_type = "String(Some(255))")]
	pub title: String,
	#[sea_orm(column_type = "Timestamp")]
	pub last_checked: DateTimeUtc,
	#[sea_orm(column_type = "Timestamp", nullable)]
	pub need_check: Option<DateTimeUtc>,
	#[sea_orm(column_type = "Unsigned", default = 0)]
	pub check_errors: u32,
	#[sea_orm(column_type = "Unsigned", default = 0)]
	pub issues: u32,
	#[sea_orm(column_type = "Unsigned", default = 0)]
	pub suggests: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::issue::Entity")]
	Issue,
}

impl Related<super::issue::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Issue.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
