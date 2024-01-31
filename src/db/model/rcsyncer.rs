use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Hash)]
#[sea_orm(table_name = "rc_syncer")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false, unique, indexed)]
	pub id: Uuid,
	#[sea_orm(column_type = "Timestamp")]
	pub last_synced_at: DateTimeUtc,
	#[sea_orm(column_type = "Unsigned", default = 0)]
	pub last_rc_id: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
