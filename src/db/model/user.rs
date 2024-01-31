use std::fmt::Display;

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false, unique, indexed)]
	pub id: Uuid,
	#[sea_orm(column_type = "String(None)")]
	pub name: String,
	#[sea_orm(column_type = "String(None)")]
	pub salt: String,
	#[sea_orm(column_type = "String(None)")]
	pub modrinth_id: String,
	#[sea_orm(column_type = "Boolean", default_value = "false")]
	pub sysop: bool,
	#[sea_orm(column_type = "Timestamp", nullable, default_value = "None")]
	pub blocked: Option<DateTimeUtc>,
}

impl Display for Model {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.id))
	}
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
