use askama::Template;
use axum::{
	routing::{get, post},
	Router,
};
use tracing::{error, info};

use crate::{app::App, page::Page};

use super::{
	auth::{AuthResult, RequireSysop},
	meta::MessagePage,
};

#[derive(Template)]
#[template(path = "sysop/info.html")]
struct InfoPage {
	auth: AuthResult,
}

pub fn new_router() -> Router {
	Router::new()
		.route(
			"/",
			get(|RequireSysop(auth): RequireSysop| async { InfoPage { auth } }),
		)
		.route(
			"/trigger-syncers",
			post(|RequireSysop(auth): RequireSysop| async {
				App::get().resync_pages_notify.notify_waiters();
				MessagePage {
					auth,
					title: "Syncers Triggerred",
					message: "All wiki page synchronizers are triggerred.",
					auto_return: true,
				}
			}),
		)
		.route(
			"/reset-login-lru",
			post(|RequireSysop(auth): RequireSysop| async {
				App::get().login_lru.write().clear();
				MessagePage {
					auth,
					title: "Login LRU Reseted",
					message: "Login token LRU cache is invalidated.",
					auto_return: true,
				}
			}),
		)
		.route(
			"/trigger-linter-worker",
			post(|RequireSysop(auth): RequireSysop| async {
				App::get().linter_notify.notify_waiters();
				MessagePage {
					auth,
					title: "Linter Triggerred",
					message: "We notified linter worker to work.",
					auto_return: true,
				}
			}),
		)
		.route(
			"/recheck-all-pages",
			post(|RequireSysop(auth): RequireSysop| async {
				info!(%auth, "mark all pages for re-check");
				tokio::spawn(async {
					if let Err(error) = Page::mark_all_pages_for_check().await {
						error!(%error, "failed to mark all pages for re-check");
					}
				});
				MessagePage {
					auth,
					title: "Recheck Triggerred",
					message: "We started to mark pages for re-check.",
					auto_return: true,
				}
			}),
		)
}
