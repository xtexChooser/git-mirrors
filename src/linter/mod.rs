use std::{
	env,
	sync::{Arc, LazyLock},
};

use anyhow::{bail, Result};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;
use tracing::{error, info_span, Instrument};
use uuid::Uuid;

use crate::{app::App, page::Page};

pub static LINTER_WORKERS: LazyLock<u32> = LazyLock::new(|| {
	env::var("SPOCK_LINTER_WORKERS")
		.ok()
		.and_then(|s| s.parse::<u32>().ok())
		.unwrap_or(5)
});

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LinterState {
	pub page: Option<Uuid>,
}

impl LinterState {
	pub fn new() -> Self {
		Self { page: None }
	}
}

pub async fn run_linters() {
	let app = App::get();
	let _ = app.mwbot("zh").await.unwrap();
	let _ = app.mwbot("en").await.unwrap();

	let mut handles = JoinSet::new();
	for _ in 0..*LINTER_WORKERS {
		let state = Arc::new(RwLock::new(LinterState::new()));
		app.linters.write().push(state.clone());
		handles.spawn(run_linter(state));
	}

	loop {
		tokio::time::sleep(std::time::Duration::from_secs(120)).await;
		match Page::count_for_check().await.unwrap_or(0) {
			0 => {}
			1 => app.linter_notify.notify_one(),
			_ => app.linter_notify.notify_waiters(),
		}
	}
}

async fn select_page(state: &RwLock<LinterState>) -> Result<Option<Page>> {
	let app = App::get();
	let _linters_lock = app.linter_selector_mutex.lock();
	let other_pages = app
		.linters
		.read()
		.iter()
		.filter_map(|l| l.read().page.to_owned())
		.collect::<Vec<_>>();
	let page = Page::find_for_check()
		.await?
		.into_iter()
		.filter(|s| !other_pages.contains(s.id()))
		.next();
	if let Some(page) = &page {
		state.write().page = Some(page.id().to_owned());
	}
	Ok(page)
}

pub async fn run_linter(state: Arc<RwLock<LinterState>>) {
	let app = App::get();
	loop {
		app.linter_notify.notified().await;
		loop {
			assert!(state.read().page.is_none());
			let page = select_page(&*state).await;
			match page {
				Err(error) => error!(%error,"error selecting page for linting"),
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
					.await
				}
				Ok(None) => {}
			}
		}
	}
}

pub async fn do_lint(id: Uuid) -> Result<(u32, u32)> {
	bail!("not implemented")
}
