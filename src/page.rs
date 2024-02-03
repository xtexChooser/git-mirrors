use std::{collections::BTreeSet, fmt::Display};

use anyhow::{anyhow, bail, Result};
use chrono::{DateTime, Duration, Utc};
use mwbot::generators::{AllPages, Generator};
use sea_orm::{
	prelude::*, ActiveValue, Condition, FromQueryResult, IntoActiveModel,
	QuerySelect,
};
use tokio::sync::Mutex;
use tracing::{error, info, info_span, trace, Instrument};
use uuid::Uuid;

use crate::{
	app::App,
	db::{self},
	linter::CONFIG_LINTER_WORKERS,
	site,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Page(db::page::Model);

impl PartialOrd for Page {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Page {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.id.cmp(&other.0.id)
	}
}

impl Display for Page {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.id.fmt(f)
	}
}

static CREATION_LOCK: Mutex<()> = Mutex::const_new(());

impl Page {
	pub async fn normalize_title(lang: &str, title: &str) -> Result<String> {
		let bot = App::get().mwbot(lang).await?;
		let codec = bot.title_codec();
		Ok(codec.to_pretty(&codec.new_title(title)?))
	}

	pub fn get_lang_id(lang: &str) -> Uuid {
		Uuid::new_v5(&site::ROOT_UUID_NS, lang.as_bytes())
	}

	pub fn get_page_id(lang: &str, title: &str) -> Uuid {
		Uuid::new_v5(&Self::get_lang_id(lang), title.as_bytes())
	}

	pub async fn get_by_id(id: &Uuid) -> Result<Option<Self>> {
		Ok(db::page::Entity::find_by_id(*id)
			.one(&*db::get())
			.await?
			.map(Page))
	}

	pub async fn get_by_name(lang: &str, title: &str) -> Result<Option<Self>> {
		Self::get_by_id(&Self::get_page_id(lang, title)).await
	}

	pub async fn get_or_init(lang: &str, title: &str) -> Result<Option<Self>> {
		if let Some(page) = Self::get_by_name(lang, title).await? {
			Ok(Some(page))
		} else {
			let _ = CREATION_LOCK.lock().await;
			if let Some(page) = Self::get_by_name(lang, title).await? {
				return Ok(Some(page));
			}

			// check language and namespace allowed
			if let Some(ns) = site::ALLOWED_NAMESPACES.get(lang) {
				let bot = App::get().mwbot(lang).await?;
				let namespace = bot
					.namespace_name(
						bot.title_codec().new_title(title)?.namespace(),
					)
					.ok_or_else(|| {
						anyhow!("name of NS of '{}' not found", title)
					})?;
				if !ns.contains(namespace) {
					return Ok(None);
				}
			} else {
				return Ok(None);
			}

			let new = db::page::ActiveModel {
				id: ActiveValue::Set(Self::get_page_id(lang, title)),
				lang: ActiveValue::Set(lang.to_owned()),
				title: ActiveValue::Set(title.to_owned()),
				need_check: ActiveValue::Set(Some(Utc::now().naive_utc())),
				..Default::default()
			};
			Ok(Some(Self(new.insert(&*db::get()).await?)))
		}
	}

	pub fn id(&self) -> &Uuid {
		&self.0.id
	}

	pub fn lang(&self) -> &str {
		&self.0.lang
	}

	pub fn title(&self) -> &str {
		&self.0.title
	}

	pub fn last_checked(&self) -> Option<DateTime<Utc>> {
		if self.0.last_checked.and_utc() == DateTime::UNIX_EPOCH {
			None
		} else {
			Some(self.0.last_checked.and_utc())
		}
	}

	pub fn check_requested_time(&self) -> Option<DateTime<Utc>> {
		self.0.need_check.map(|ts| ts.and_utc())
	}

	pub fn check_errors(&self) -> u32 {
		self.0.check_errors as u32
	}

	pub fn issues_count(&self) -> u32 {
		self.0.issues as u32
	}

	pub fn suggests_count(&self) -> u32 {
		self.0.suggests as u32
	}

	pub async fn mark_check(self) -> Result<()> {
		let mut model = self.0.into_active_model();
		model.need_check = ActiveValue::Set(Some(Utc::now().naive_utc()));
		model.check_errors = ActiveValue::Set(0);
		model.update(&*db::get()).await?;
		App::get().linter.worker_notify.notify_one();
		Ok(())
	}

	pub async fn set_checked(
		self,
		start_time: DateTime<Utc>,
		issues: u32,
		suggests: u32,
	) -> Result<()> {
		let requested_time = self.check_requested_time();
		let drop_result = if requested_time != Some(start_time) {
			info!(
				lang = self.lang(),
				title = self.title(),
				"drop check result for page to be checked again"
			);
			true
		} else {
			info!(
				lang = self.lang(),
				title = self.title(),
				time = %(Utc::now() - start_time),
				issues,
				suggests,
				"succeeded checking page"
			);
			false
		};
		let mut model = self.0.into_active_model();
		model.last_checked = ActiveValue::Set(Utc::now().naive_utc());
		if !drop_result {
			model.need_check = ActiveValue::Set(None);
		}
		model.check_errors = ActiveValue::Set(0);
		model.issues = ActiveValue::Set(issues as i32);
		model.suggests = ActiveValue::Set(suggests as i32);
		model.update(&*db::get()).await?;
		Ok(())
	}

	pub async fn defer_check(self) -> Result<()> {
		let check_time = self.check_requested_time().map(|ts| ts.naive_utc());
		let check_errors = self.check_errors();
		let mut model = self.0.into_active_model();
		model.need_check = ActiveValue::Set(Some(
			check_time.ok_or_else(|| {
				anyhow!("trying to defer a page not requested for checking")
			})? + Duration::seconds(site::LINTER_RETRY_DELAY),
		));
		model.check_errors = ActiveValue::Set(check_errors as i32 + 1);
		model.update(&*db::get()).await?;
		Ok(())
	}

	pub async fn delete(self) -> Result<()> {
		self.0.into_active_model().delete(&*db::get()).await?;
		Ok(())
	}

	pub async fn mark_all_pages_for_check() -> Result<()> {
		info!("marking all pages for check");
		db::page::Entity::update_many()
			.set(db::page::ActiveModel {
				need_check: ActiveValue::Set(Some(Utc::now().naive_utc())),
				check_errors: ActiveValue::Set(0),
				..Default::default()
			})
			.exec(&*db::get())
			.await?;
		App::get().linter.worker_notify.notify_waiters();
		Ok(())
	}

	pub async fn mark_error_pages_for_check() -> Result<()> {
		info!("marking error pages for check");
		db::page::Entity::update_many()
			.set(db::page::ActiveModel {
				need_check: ActiveValue::Set(Some(Utc::now().naive_utc())),
				check_errors: ActiveValue::Set(0),
				..Default::default()
			})
			.filter(db::page::Column::CheckErrors.gte(site::LINTER_MAX_RETRIES))
			.exec(&*db::get())
			.await?;
		App::get().linter.worker_notify.notify_waiters();
		Ok(())
	}

	pub async fn count_for_check() -> Result<u64> {
		Ok(db::page::Entity::find()
			.filter(
				Condition::all()
					.add(db::page::Column::NeedCheck.is_not_null())
					.add(
						db::page::Column::NeedCheck.lte(Utc::now().naive_utc()),
					)
					.add(
						db::page::Column::CheckErrors
							.lt(site::LINTER_MAX_RETRIES),
					),
			)
			.count(&*db::get())
			.await?)
	}

	pub async fn find_for_check() -> Result<Vec<Self>> {
		Ok(db::page::Entity::find()
			.filter(
				Condition::all()
					.add(db::page::Column::NeedCheck.is_not_null())
					.add(
						db::page::Column::NeedCheck.lte(Utc::now().naive_utc()),
					)
					.add(
						db::page::Column::CheckErrors
							.lt(site::LINTER_MAX_RETRIES),
					),
			)
			.limit(Some(*CONFIG_LINTER_WORKERS as u64 * 2))
			.all(&*db::get())
			.await?
			.into_iter()
			.map(Self)
			.collect())
	}
}

pub async fn sync_all_pages(lang: &str) -> Result<()> {
	let app = App::get();
	let bot = app.mwbot(lang).await?;
	let mut pages: BTreeSet<String> = BTreeSet::new();

	for ns in &site::ALLOWED_NAMESPACES[lang] {
		let nsid = bot
			.namespace_id(*ns)
			.ok_or_else(|| anyhow!("NS {} does not exist on {}", ns, lang))?;
		if nsid < 0 {
			bail!("NS {} on {} is special NS", ns, lang);
		}
		let mut gen = AllPages::new(nsid as u32).generate(&bot);
		while let Some(page) = gen.recv().await {
			let page = page?;
			trace!(lang, ns, %page, "syncing page");
			pages.insert(page.title().to_string());

			let ts = *page.touched().await?.ok_or_else(|| {
				anyhow!("page does not exist is generated by allpages")
			})?;
			let mut dbpage = Page::get_by_name(lang, page.title()).await?;
			if dbpage.is_none() {
				info!(lang, ns, %page, "find new page in allpages sync");
				dbpage = Page::get_or_init(lang, page.title()).await?;
			}
			let dbpage = dbpage.ok_or_else(|| {
				anyhow!("allpages syncer got a disallowed page")
			})?;
			if let Some(t) = dbpage.last_checked() {
				if ts > t {
					info!(lang, ns, %page, "page outdated, marking for check");
					dbpage.mark_check().await?;
				}
			}
		}
	}

	#[derive(Debug, FromQueryResult)]
	#[sea_orm(entity = "db::page::Entity")]
	pub struct TitleOnlyModel {
		pub id: Uuid,
		pub title: String,
	}

	let dbpages = db::page::Entity::find()
		.filter(db::page::Column::Lang.eq(lang))
		.select_only()
		.column(db::page::Column::Id)
		.column(db::page::Column::Title)
		.into_model::<TitleOnlyModel>()
		.all(&*db::get())
		.await?;
	for dbpage in dbpages {
		if !pages.contains(&dbpage.title) {
			info!(page = dbpage.title, "remove deleted page from database");
			if let Some(page) = Page::get_by_id(&dbpage.id).await? {
				page.delete().await?;
			}
		}
	}

	Ok(())
}

pub async fn run_page_list_syncer() {
	let app = App::get();

	loop {
		tokio::select! {
			_ = app.resync_pages_notify.notified()=>{},
			_ = tokio::time::sleep(std::time::Duration::from_secs(site::SYNC_ALL_PAGES_PEROID))=>{}
		}
		for lang in site::ALLOWED_NAMESPACES.keys() {
			if let Err(error) = sync_all_pages(lang)
				.instrument(info_span!("sync_all_pages", lang))
				.await
			{
				error!(?error, lang, "failed to sync all pages");
			}
		}
	}
}
