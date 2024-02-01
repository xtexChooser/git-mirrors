pub mod prelude {
	pub use super::{
		super::{LinterState, WorkerState},
		CheckContext, CheckResult, CheckerId, CheckerTrait,
	};
	pub use crate::issue::*;
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
	collections::HashMap,
	fmt::{Debug, Display},
	sync::Arc,
};

use prelude::*;

use crate::{issue::Issue, page::Page};

#[async_trait]
pub trait CheckerTrait
where
	Self: Debug + Display + Send + Sync + CheckerId,
{
	fn possible_issues(&self) -> Vec<Issue>;
	async fn check(&self, ctx: &mut CheckContext) -> CheckResult;
}

pub type Checker = Arc<Box<dyn CheckerTrait>>;

pub trait CheckerId {
	fn get_id(&self) -> &'static str;
}

#[macro_export]
macro_rules! checker {
	($typ: ident, $id: expr) => {
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

		impl crate::linter::checker::CheckerId for $typ {
			fn get_id(&self) -> &'static str {
				$id
			}
		}
	};
}

#[macro_export]
macro_rules! checkers {
	[$($typ: ident),+] => {
		vec![
			$( Arc::new(Box::new($typ ())) ),+
		]
	};
}

pub struct CheckContext {
	pub id: Uuid,
	pub lang: String,
	pub title: String,
	pub app: Arc<App>,
	pub bot: MwBot,
	pub page: MwPage,
	pub found_issues: HashMap<String, serde_json::Value>,
	pub found_suggests: HashMap<String, serde_json::Value>,
}

impl CheckContext {
	pub async fn new(id: Uuid) -> Result<Self> {
		let dbpage = Page::get_by_id(&id)
			.await?
			.ok_or_else(|| anyhow!("page id for CheckContext does not exist"))?;
		let app = App::get();
		let bot = app.mwbot(dbpage.lang()).await?;
		let page = bot.page(dbpage.title())?;
		Ok(Self {
			id,
			lang: dbpage.lang().to_owned(),
			title: dbpage.title().to_owned(),
			app,
			bot,
			page,
			found_issues: HashMap::new(),
			found_suggests: HashMap::new(),
		})
	}

	pub fn issue<S>(&mut self, checker: &dyn CheckerId, issue: S) -> Result<()>
	where
		S: Serialize,
	{
		self.found_issues
			.insert(checker.get_id().to_string(), serde_json::to_value(issue)?);
		Ok(())
	}

	pub fn suggest<S>(&mut self, checker: &dyn CheckerId, issue: S) -> Result<()>
	where
		S: Serialize,
	{
		self.found_suggests
			.insert(checker.get_id().to_string(), serde_json::to_value(issue)?);
		Ok(())
	}
}

pub type CheckResult = Result<()>;
