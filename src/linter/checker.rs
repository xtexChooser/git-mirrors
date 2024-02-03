pub mod prelude {
	pub use crate::issue::prelude::*;
	pub use crate::linter::checker::{
		CheckContext, CheckResource, CheckResult, Checker, CheckerId,
		CheckerTrait,
	};
	pub use crate::linter::{LinterState, WorkerState};
	pub use crate::{
		app::{App, MwBot, MwPage},
		checker, site,
	};
}

use std::{
	any::{type_name, Any, TypeId},
	collections::HashMap,
	fmt::{Debug, Display},
	sync::Arc,
};

use parking_lot::Mutex;
use prelude::*;
use tokio::sync::RwLock;

use crate::{issue::IssueType, page::Page};

#[async_trait]
pub trait CheckerTrait
where
	Self: Debug + Display + Send + Sync + CheckerId,
{
	fn possible_issues(&self) -> Vec<IssueType>;
	async fn check(&self, ctx: Arc<CheckContext>) -> CheckResult;
}

pub type Checker = Arc<Box<dyn CheckerTrait>>;

pub trait CheckerId {
	fn get_id(&self) -> &'static str;
	fn get_type_id(&self) -> TypeId;
}

#[macro_export]
macro_rules! checker {
	($typ: ident, $id: literal) => {
		::paste::paste! {
			pub struct [<$typ Checker>];

			impl std::fmt::Debug for [<$typ Checker>] {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					f.write_str($id)
				}
			}

			impl std::fmt::Display for [<$typ Checker>] {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					f.write_str($id)
				}
			}

			impl $crate::linter::checker::CheckerId for [<$typ Checker>] {
				fn get_id(&self) -> &'static str {
					$id
				}
				fn get_type_id(&self) -> std::any::TypeId {
					std::any::TypeId::of::<[<$typ Checker>]>()
				}
			}

			impl Default for [<$typ Checker>] {
				fn default() -> Self {
					Self {}
				}
			}
		}
	};
}

#[macro_export]
macro_rules! checkers {
	[$($typ: ident),+] => {
		vec![
			$( ::paste::paste! { std::sync::Arc::new(Box::new([<$typ Checker>] {})) } ),+
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
	pub found_issues: Mutex<Vec<(IssueType, serde_json::Value)>>,
	// only insert and update, never remove from resources
	resources: RwLock<HashMap<TypeId, CheckResource>>,
}

pub type CheckResource = Box<Arc<dyn Any + Send + Sync>>;

impl CheckContext {
	pub async fn new(id: Uuid) -> Result<Self> {
		let dbpage = Page::get_by_id(&id).await?.ok_or_else(|| {
			anyhow!("page id for CheckContext does not exist")
		})?;
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
			found_issues: Mutex::new(Vec::new()),
			resources: RwLock::new(HashMap::new()),
		})
	}

	pub fn found<I, S>(&self, issue: S) -> Result<()>
	where
		I: IssueTrait + Default + 'static,
		S: Serialize,
	{
		self.found_issues.lock().push((
			Arc::new(Box::<I>::default()),
			serde_json::to_value(issue)?,
		));
		Ok(())
	}

	pub async fn resource<T: 'static + Send + Sync>(&self) -> Result<Arc<T>> {
		let resources = self.resources.read().await;
		let res = resources.get(&TypeId::of::<T>()).ok_or_else(|| {
			anyhow!("resource {} is not initialized yet", type_name::<T>())
		})?;
		let res = res.as_ref().to_owned();
		let res = unsafe { res.downcast_unchecked::<T>() };
		Ok(res)
	}

	pub async fn insert_resource_arc<T: 'static + Send + Sync>(
		&self,
		value: Arc<T>,
	) {
		self.resources
			.write()
			.await
			.insert(TypeId::of::<T>(), Box::new(value));
	}

	pub async fn insert_resource<T: 'static + Send + Sync>(&self, value: T) {
		self.insert_resource_arc::<T>(Arc::new(value)).await;
	}

	pub async fn compute_resource<
		T: 'static + Send + Sync + ComputedResource,
	>(
		self: &Arc<Self>,
	) -> Result<Arc<T>> {
		let key = TypeId::of::<T>();
		let exist = self.resources.read().await.contains_key(&key);
		if !exist {
			if let std::collections::hash_map::Entry::Vacant(e) =
				self.resources.write().await.entry(key)
			{
				e.insert(Box::new(Arc::new(T::compute(self.clone()).await?)));
			}
		}
		self.resource::<T>().await
	}
}

pub type CheckResult = Result<()>;

#[async_trait]
pub trait ComputedResource
where
	Self: 'static + Send + Sync + Sized,
{
	async fn compute(ctx: Arc<CheckContext>) -> Result<Self>;
}
