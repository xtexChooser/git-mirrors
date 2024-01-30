use std::collections::HashMap;

use askama::Template;
use axum::{
	response::IntoResponse,
	routing::{get, post},
	Router,
};
use sea_orm::{
	EntityTrait, PaginatorTrait, QueryOrder,
};
use tracing::{error, info};

use crate::{app::App, db, page::Page, site};

use super::{
	auth::{AuthResult, RequireSysop},
	meta::MessagePage,
	WebResult,
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
		.route("/rcsyncer", get(rcsyncer_state_handler))
		.route("/stats", get(stats_handler))
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

#[derive(Template)]
#[template(path = "sysop/rcsyncer.html")]
struct RcSyncerPage {
	auth: AuthResult,
	state: Vec<(String, db::rcsyncer::Model)>,
}

async fn rcsyncer_state_handler(RequireSysop(auth): RequireSysop) -> WebResult {
	let mut langs = HashMap::new();
	for lang in &site::SYNC_RC {
		langs.insert(Page::get_lang_id(lang), lang);
	}
	Ok(RcSyncerPage {
		auth,
		state: db::rcsyncer::Entity::find()
			.order_by_asc(db::rcsyncer::Column::Id)
			.all(&*db::get())
			.await?
			.into_iter()
			.map(|state| (langs[&state.id].to_string(), state))
			.collect(),
	}
	.into_response())
}

#[derive(Template)]
#[template(path = "sysop/stats.html")]
struct StatsPage {
	auth: AuthResult,
	page_count: u64,
	issue_count: u64,
	user_count: u64,
}

async fn stats_handler(RequireSysop(auth): RequireSysop) -> WebResult {
	Ok(StatsPage {
		auth,
		page_count: db::page::Entity::find().count(&*db::get()).await?,
		issue_count: db::issue::Entity::find().count(&*db::get()).await?,
		user_count: db::user::Entity::find().count(&*db::get()).await?,
	}
	.into_response())
}
