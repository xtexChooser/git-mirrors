use std::{
	env,
	sync::{Arc, LazyLock},
};

use anyhow::{bail, Result};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, Notify};
use tracing::{error, info_span, Instrument};
use uuid::Uuid;

use crate::{app::App, page::Page};

pub static LINTER_WORKERS: LazyLock<u32> = LazyLock::new(|| {
	env::var("SPOCK_LINTER_WORKERS")
		.ok()
		.and_then(|s| s.parse::<u32>().ok())
		.unwrap_or(5)
});

#[derive(Debug, Default)]
pub struct LinterState {
	pub worker_notify: Notify,
	pub selector_mutex: Mutex<()>,
	pub workers: RwLock<Vec<Arc<RwLock<WorkerState>>>>,
}

impl LinterState {
	pub fn new() -> Self {
		Self::default()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
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

	for _ in 0..*LINTER_WORKERS {
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
				Err(error) => error!(%error, "error selecting page for linting"),
				Ok(Some(page)) => {
					let title = page.title().to_owned();
					async {
						let start_time = page
							.check_requested_time()
							.expect("select_page returned a page that is not requested for check");
						match do_lint(page.id().to_owned()).await {
							Ok((issues, suggests)) => {
								if let Err(error) =
									page.set_checked(start_time, issues, suggests).await
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

pub async fn do_lint(id: Uuid) -> Result<(u32, u32)> {
	bail!("not implemented")
}
