use std::{
	collections::{BTreeMap, HashMap},
	sync::Arc,
};

use anyhow::{bail, Context, Result};
use parking_lot::RwLock;
use sea_orm::{
	ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter,
	TransactionTrait,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, Notify};
use tracing::{error, info, info_span, Instrument};
use uuid::Uuid;

use crate::{
	app::App, config, db, issue::IssueType, linter::checker::CheckContext,
	page::Page, site,
};

use self::checker::Checker;

config!(LINTER_WORKERS, parse u32, 5);

pub mod checker;
pub mod generic;

#[derive(Debug, Default)]
pub struct LinterState {
	pub worker_notify: Notify,
	pub selector_mutex: Mutex<()>,
	pub workers: RwLock<Vec<Arc<RwLock<WorkerState>>>>,
	pub checkers: BTreeMap<String, Checker>,
	pub issues: BTreeMap<String, IssueType>,
}

impl LinterState {
	pub fn new() -> Result<Self> {
		let checkers = Self::init_checkers()?;
		let issues = Self::init_issues(&checkers)?;
		Ok(Self {
			worker_notify: Notify::new(),
			selector_mutex: Mutex::default(),
			workers: RwLock::new(Vec::new()),
			checkers,
			issues,
		})
	}

	fn init_checkers() -> Result<BTreeMap<String, Checker>> {
		let mut checkers = BTreeMap::new();
		for checker in site::init_checkers() {
			let id = checker.get_id();
			let type_id = checker.get_type_id();
			if let Some(prev) = checkers.insert(id.to_string(), checker) {
				if prev.get_type_id() != type_id {
					bail!("found different checker type with same ID: {}", id);
				}
			}
		}
		Ok(checkers)
	}

	fn init_issues(
		checkers: &BTreeMap<String, Checker>,
	) -> Result<BTreeMap<String, IssueType>> {
		let mut issues = BTreeMap::new();
		for checker in checkers.values() {
			for issue in checker.possible_issues() {
				let id = issue.get_id();
				let type_id = issue.get_type_id();
				if let Some(prev) = issues.insert(id.to_string(), issue) {
					if prev.get_type_id() != type_id {
						bail!(
							"found different issue type with same ID: {}",
							id
						);
					}
				}
			}
		}
		Ok(issues)
	}
}

#[derive(
	Debug,
	Serialize,
	Deserialize,
	Clone,
	Hash,
	PartialEq,
	Eq,
	PartialOrd,
	Ord,
	Default,
)]
pub struct WorkerState {
	pub page: Option<Uuid>,
}

impl WorkerState {
	pub fn new() -> Self {
		Self::default()
	}
}

pub async fn run_linters() {
	let app = App::get();
	let _ = app.mwbot("zh").await.unwrap();
	let _ = app.mwbot("en").await.unwrap();

	for _ in 0..*CONFIG_LINTER_WORKERS {
		let state = Arc::new(RwLock::new(WorkerState::new()));
		app.linter.workers.write().push(state.clone());
		tokio::spawn(run_linter(state));
	}

	loop {
		tokio::time::sleep(std::time::Duration::from_secs(120)).await;
		match Page::count_for_check().await.unwrap_or(0) {
			0 => {}
			1 => app.linter.worker_notify.notify_one(),
			_ => app.linter.worker_notify.notify_waiters(),
		}
	}
}

// @TODO: cache a part of pages
async fn select_page(state: &RwLock<WorkerState>) -> Result<Option<Page>> {
	let app = App::get();
	let _linters_lock = app.linter.selector_mutex.lock();
	let other_pages = app
		.linter
		.workers
		.read()
		.iter()
		.filter_map(|l| l.read().page.to_owned())
		.collect::<Vec<_>>();
	let page = Page::find_for_check()
		.await?
		.into_iter()
		.find(|s| !other_pages.contains(s.id()));
	if let Some(page) = &page {
		state.write().page = Some(page.id().to_owned());
	}
	Ok(page)
}

pub async fn run_linter(state: Arc<RwLock<WorkerState>>) {
	let app = App::get();
	loop {
		app.linter.worker_notify.notified().await;
		loop {
			assert!(state.read().page.is_none());
			let page = select_page(&state).await;
			match page {
				Err(error) => {
					error!(%error, "error selecting page for linting")
				}
				Ok(Some(page)) => {
					let title = page.title().to_owned();
					async {
						let start_time = page
							.check_requested_time()
							.expect("select_page returned a page that is not requested for check");
						match do_lint(page.id().to_owned()).await {
							Ok((issues, suggestions)) => {
								if let Err(error) = page
									.set_checked(
										start_time,
										issues,
										suggestions,
									)
									.await
								{
									error!(%error, "failed to mark page as checked");
								}
							}
							Err(error) => {
								error!(%error, %page, "failed to check page");
								if let Err(error) = page.defer_check().await {
									error!(%error, "failed to defer checking page");
								}
							}
						}
					}
					.instrument(info_span!("lint_page", page = title))
					.await;
					state.write().page = None;
				}
				Ok(None) => break,
			}
		}
	}
}

pub async fn do_lint(page_id: Uuid) -> Result<(u32, u32)> {
	let ctx = Arc::new(CheckContext::new(page_id).await?);
	let app = App::get();
	let span = info_span!("check_page", page = %page_id);
	for (checker_id, checker) in &app.linter.checkers {
		if let Err(error) =
			checker.check(ctx.clone()).instrument(span.clone()).await
		{
			error!(page = %page_id, %error, checker = checker_id, "error checking page");
			return Err(error)
				.with_context(|| format!("checker: {}", checker_id));
		}
	}
	let all_issues = ctx
		.found_issues
		.lock()
		.drain(..)
		.collect::<Vec<(IssueType, serde_json::Value)>>();
	let total_issues = all_issues
		.iter()
		.filter(|(i, _)| i.get_level().is_issue())
		.count();
	let total_suggestions = all_issues.len() - total_issues;

	// upload issues
	let txn = db::get().begin().await?;
	let mut exist = db::issue::Entity::find()
		.filter(db::issue::Column::Page.eq(page_id))
		.all(&txn)
		.await?
		.into_iter()
		.map(|i| (i.id.to_owned(), i))
		.collect::<HashMap<_, _>>();
	for (typ, val) in all_issues {
		let val_json = serde_json::to_string(&val)?;
		let id = Uuid::new_v5(
			&page_id,
			&[typ.get_id().as_bytes(), val_json.as_bytes()].concat(),
		);
		if let Some(model) = exist.remove(&id) {
			if model.issue_type != typ.get_id() || model.details != val {
				bail!("hash conflict found: {} {}", page_id, typ);
			}
		} else {
			// new issue
			info!(page = %page_id, issue = %id, issue_type = %typ, details = val_json, "found issue");
			let model = db::issue::ActiveModel {
				id: ActiveValue::Set(id),
				page: ActiveValue::Set(page_id),
				issue_type: ActiveValue::Set(typ.get_id().to_string()),
				details: ActiveValue::Set(val),
			};
			model.insert(&txn).await?;
		}
	}
	for id in exist.into_keys() {
		info!(page = %page_id, issue = %id, "remove issue");
		db::issue::Entity::delete_by_id(id).exec(&txn).await?;
	}
	txn.commit().await?;

	Ok((total_issues as u32, total_suggestions as u32))
}
