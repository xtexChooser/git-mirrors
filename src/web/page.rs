use std::collections::HashMap;

use askama::Template;
use axum::{
	extract::{Path, Query},
	response::{IntoResponse, Redirect},
	routing::get,
	Router,
};
use sea_orm::{
	ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter,
	QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db, page::Page};

use super::{auth::AuthResult, WebResult};

#[derive(Template)]
#[template(path = "linter/list.html")]
struct ListPage {
	auth: AuthResult,
	page: u64,
	pages: Vec<db::page::Model>,
}

pub fn new_router() -> Router {
	Router::new()
		.route("/", get(list_handler))
		.route("/:id", get(get_handler))
		.route("/:lang/:title", get(get_title_handler))
}

#[derive(Debug, Serialize, Deserialize)]
struct ListParams {
	#[serde(default)]
	pub page: u64,
	#[serde(default)]
	pub lang: Option<String>,
	#[serde(default)]
	pub issues_count: Option<u32>,
	#[serde(default)]
	pub suggestions_count: Option<u32>,
}

async fn list_handler(
	auth: AuthResult,
	Query(params): Query<ListParams>,
) -> WebResult {
	let page = params.page;
	let mut filter = Condition::all();
	if let Some(lang) = params.lang {
		filter = filter.add(db::page::Column::Lang.eq(lang));
	}
	if let Some(issues_count) = params.issues_count {
		filter = filter.add(db::page::Column::Issues.gte(issues_count));
	}
	if let Some(suggestions_count) = params.suggestions_count {
		filter = filter.add(db::page::Column::Suggests.gte(suggestions_count));
	}
	let pages = db::page::Entity::find()
		.filter(filter)
		.order_by_desc(db::page::Column::Issues)
		.paginate(&*db::get(), 100)
		.fetch_page(page)
		.await?;
	Ok(ListPage { auth, page, pages }.into_response())
}

#[derive(Template)]
#[template(path = "linter/page.html")]
struct PageInfoPage {
	auth: AuthResult,
	page: db::page::Model,
	issues: HashMap<String, Vec<db::issue::Model>>,
}

async fn get_handler(auth: AuthResult, Path(page_id): Path<Uuid>) -> WebResult {
	let (page, issues) = db::page::Entity::find_by_id(page_id)
		.find_with_related(db::issue::Entity)
		.limit(1)
		.all(&*db::get())
		.await?[0];
	Ok(ListPage { auth, page, issues: }.into_response())
}

async fn get_title_handler(
	Path((lang, title)): Path<(String, String)>,
) -> impl IntoResponse {
	Redirect::temporary(&format!("/page/{}", Page::get_page_id(&lang, &title)))
}
