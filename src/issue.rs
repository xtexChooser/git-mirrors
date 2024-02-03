pub mod prelude {
	pub use crate::issue::{IssueInfoTrait, IssueLevel, IssueTrait, IssueType};
	pub use crate::{
		app::{App, MwBot, MwPage},
		checker, site,
	};
	pub use anyhow::{anyhow, bail, Result};
	pub use async_trait::async_trait;
	pub use serde::{Deserialize, Serialize};
	pub use std::sync::Arc;
	pub use uuid::Uuid;
}

use std::{
	any::TypeId,
	fmt::{Debug, Display},
};

use prelude::*;

#[async_trait]
pub trait IssueTrait
where
	Self: Debug + Display + Send + Sync + IssueInfoTrait,
{
}

pub type IssueType = Arc<Box<dyn IssueTrait>>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Copy)]
pub enum IssueLevel {
	Issue,
	Suggestion,
}

impl IssueLevel {
	pub fn is_issue(&self) -> bool {
		*self == Self::Issue
	}
	pub fn is_suggestion(&self) -> bool {
		*self == Self::Suggestion
	}
}

pub trait IssueInfoTrait {
	fn get_id(&self) -> &'static str;
	fn get_level(&self) -> IssueLevel;
	fn get_type_id(&self) -> TypeId;
}

#[macro_export]
macro_rules! declare_issue {
	($typ: ident, $id: literal, $level: ident) => {
		::paste::paste! {
			pub struct [<$typ Issue>];

			impl std::fmt::Debug for [<$typ Issue>] {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					f.write_str($id)
				}
			}

			impl std::fmt::Display for [<$typ Issue>] {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					f.write_str($id)
				}
			}

			impl $crate::issue::IssueInfoTrait for [<$typ Issue>] {
				fn get_id(&self) -> &'static str {
					$id
				}
				fn get_level(&self) -> IssueLevel {
					IssueLevel::$level
				}
				fn get_type_id(&self) -> std::any::TypeId {
					std::any::TypeId::of::<[<$typ Issue>]>()
				}
			}

			impl Default for [<$typ Issue>] {
				fn default() -> Self {
					Self {}
				}
			}
		}
	};
}

#[macro_export]
macro_rules! issue {
	($typ: ident, $id: expr) => {
		$crate::declare_issue!($typ, $id, Issue);
	};
}

#[macro_export]
macro_rules! suggestion {
	($typ: ident, $id: expr) => {
		$crate::declare_issue!($typ, $id, Suggestion);
	};
}

#[macro_export]
macro_rules! issues {
	[$($typ: ident),*] => {
		vec![
			$( ::paste::paste! { std::sync::Arc::new(Box::new([<$typ Issue>]{})) } ),*
		]
	};
}
