use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, DeriveEntityModel)]
#[sea_orm(table_name = "page")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub lang: String,
	pub title: String,
	pub last_checked: DateTimeUtc,
	pub need_check: Option<DateTimeUtc>,
	pub issues: u32,
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
