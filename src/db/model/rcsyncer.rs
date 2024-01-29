use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Hash)]
#[sea_orm(table_name = "rc_syncer")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub last_synced_at: DateTimeUtc,
	pub last_rc_id: u32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
