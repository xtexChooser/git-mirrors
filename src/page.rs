use anyhow::Result;
use chrono::{DateTime, Utc};
use sea_orm::{prelude::*, ActiveValue, IntoActiveModel};
use tokio::sync::Mutex;
use tracing::info;
use uuid::{uuid, Uuid};

use crate::{
	app::App,
	db::{self},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Page(db::page::Model);

impl PartialOrd for Page {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.0.id.partial_cmp(&other.0.id)
	}
}

impl Ord for Page {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.id.cmp(&other.0.id)
	}
}

const UUID_NS: Uuid = uuid!("5dd8c71e-1bed-44a3-b4ec-05d08ad9c2ec");

static CREATION_LOCK: Mutex<()> = Mutex::const_new(());

impl Page {
	pub async fn normalize_title(lang: &str, title: &str) -> Result<String> {
		let bot = App::get().mwbot(lang).await?;
		let codec = bot.title_codec();
		Ok(codec.to_pretty(&codec.new_title(title)?))
	}

	pub fn get_lang_id(lang: &str) -> Uuid {
		Uuid::new_v5(&UUID_NS, lang.as_bytes())
	}

	pub fn get_page_id(lang: &str, title: &str) -> Uuid {
		Uuid::new_v5(&Self::get_lang_id(lang), title.as_bytes())
	}

	pub async fn get_by_id(id: Uuid) -> Result<Option<Self>> {
		Ok(db::page::Entity::find_by_id(id)
			.one(db::get().as_ref())
			.await?
			.map(|e| Page(e)))
	}

	pub async fn get_by_name(lang: &str, title: &str) -> Result<Option<Self>> {
		Self::get_by_id(Self::get_page_id(lang, title)).await
	}

	pub async fn get_or_init(lang: &str, title: &str) -> Result<Self> {
		if let Some(page) = Self::get_by_name(lang, title).await? {
			return Ok(page);
		} else {
			let _ = CREATION_LOCK.lock().await;
			if let Some(page) = Self::get_by_name(lang, title).await? {
				return Ok(page);
			}
			let new = db::page::ActiveModel {
				id: ActiveValue::Set(Self::get_page_id(lang, title)),
				lang: ActiveValue::Set(lang.to_owned()),
				title: ActiveValue::Set(title.to_owned()),
				..Default::default()
			};
			Ok(Self(new.insert(db::get().as_ref()).await?))
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
		if self.0.last_checked == DateTime::UNIX_EPOCH {
			None
		} else {
			Some(self.0.last_checked.to_owned())
		}
	}

	pub fn check_requested_time(&self) -> Option<DateTime<Utc>> {
		self.0.need_check
	}

	pub fn issues_count(&self) -> u32 {
		self.0.issues
	}

	pub async fn mark_check(lang: &str, title: &str) -> Result<()> {
		let mut model = Self::get_or_init(lang, title).await?.0.into_active_model();
		model.need_check = ActiveValue::Set(Some(Utc::now()));
		model.update(db::get().as_ref()).await?;
		Ok(())
	}

	pub async fn set_checked(self, start_time: DateTime<Utc>, issues: u32) -> Result<()> {
		if self.check_requested_time() != Some(start_time) {
			info!(
				lang = self.lang(),
				title = self.title(),
				"drop check result for page to be checked again"
			);
			return Ok(());
		}
		let mut model = self.0.into_active_model();
		model.last_checked = ActiveValue::Set(Utc::now());
		model.need_check = ActiveValue::Set(None);
		model.issues = ActiveValue::Set(issues);
		model.update(db::get().as_ref()).await?;
		Ok(())
	}
}
