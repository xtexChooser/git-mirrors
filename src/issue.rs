pub mod prelude {
	pub use crate::{
		app::{App, MwBot, MwPage},
		checker, site,
	};
	pub use anyhow::{anyhow, bail, Result};
	pub use async_trait::async_trait;
	pub use serde::{Deserialize, Serialize};
	pub use uuid::Uuid;
}

use std::{
	fmt::{Debug, Display},
	sync::Arc,
};

use prelude::*;

use crate::page::Page;

#[async_trait]
pub trait IssueTrait
where
	Self: Debug + Display + Send + Sync + IssueInfoTrait,
{
	fn name(&self) -> &'static str;
}

pub type Issue = Arc<Box<dyn IssueTrait>>;

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
}

#[macro_export]
macro_rules! declare_issue {
	($typ: ident, $id: expr, $level: ident) => {
		pub struct $typ();

		impl std::fmt::Debug for $typ {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str($id)
			}
		}

		impl std::fmt::Display for $typ {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				f.write_str($id)
			}
		}

		impl crate::issue::IssueInfoTrait for $typ {
			fn get_id(&self) -> &'static str {
				$id
			}
			fn get_level(&self) -> IssueLevel {
				IssueLevel::$level
			}
		}
	};
}

#[macro_export]
macro_rules! issue {
	($typ: ident, $id: expr, $level: ident) => {
		declare_issue!($typ, $id, Issue)
	};
}

#[macro_export]
macro_rules! suggestion {
	($typ: ident, $id: expr, $level: ident) => {
		declare_issue!($typ, $id, Suggestion)
	};
}

#[macro_export]
macro_rules! issues {
	[$($typ: ident),*] => {
		vec![
			$( Arc::new(Box::new($typ ())) ),*
		]
	};
}
