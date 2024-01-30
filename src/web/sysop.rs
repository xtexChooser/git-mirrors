use std::collections::HashMap;

use askama::Template;
use axum::{
	response::IntoResponse,
	routing::{get, post},
	Router,
};
use chrono::{Duration, Utc};
use sea_orm::{
	ColumnTrait, EntityTrait, FromQueryResult, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
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
	stats_time: Duration,
	pages_count: u64,
	issues_count: u64,
	users_count: u64,
	langs: Vec<LangStat>,
	need_check_count: u64,
	langs_need_check: HashMap<String, u32>,
}

#[derive(Debug, FromQueryResult)]
struct LangStat {
	lang: String,
	count: u32,
	issues: u32,
	suggests: u32,
}

#[derive(Debug, FromQueryResult)]
struct LangNeedCheckStat {
	lang: String,
	count: u32,
}

async fn stats_handler(RequireSysop(auth): RequireSysop) -> WebResult {
	let start = Utc::now();
	let pages_count = db::page::Entity::find().count(&*db::get()).await?;
	let issues_count = db::issue::Entity::find().count(&*db::get()).await?;
	let users_count = db::user::Entity::find().count(&*db::get()).await?;
	let langs = db::page::Entity::find()
		.select_only()
		.column(db::page::Column::Lang)
		.column_as(db::page::Column::Id.count(), "count")
		.column_as(db::page::Column::Issues.sum(), "issues")
		.column_as(db::page::Column::Suggests.sum(), "suggests")
		.group_by(db::page::Column::Lang)
		.into_model::<LangStat>()
		.all(&*db::get())
		.await?;
	let need_check_count = db::page::Entity::find()
		.filter(db::page::Column::NeedCheck.is_not_null())
		.count(&*db::get())
		.await?;
	let langs_need_check = db::page::Entity::find()
		.select_only()
		.column(db::page::Column::Lang)
		.column_as(db::page::Column::Id.count(), "count")
		.filter(db::page::Column::NeedCheck.is_not_null())
		.group_by(db::page::Column::Lang)
		.into_model::<LangNeedCheckStat>()
		.all(&*db::get())
		.await?
		.into_iter()
		.map(|s| (s.lang, s.count))
		.collect();
	let stats_time = Utc::now() - start;
	Ok(StatsPage {
		auth,
		stats_time,
		pages_count,
		issues_count,
		users_count,
		langs,
		need_check_count,
		langs_need_check,
	}
	.into_response())
}
