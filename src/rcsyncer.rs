use std::collections::BTreeSet;

use anyhow::Result;
use chrono::{DateTime, Utc};
use mwbot::generators::{
	recent_changes::{self, RecentChanges},
	Generator,
};
use sea_orm::{prelude::*, ActiveValue, IntoActiveModel};
use tracing::{error, info, info_span, trace, Instrument};
use uuid::Uuid;

use crate::{
	app::App,
	db::{self},
	page::Page,
	site,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RcSyncerState(db::rcsyncer::Model);

impl PartialOrd for RcSyncerState {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for RcSyncerState {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.0.id.cmp(&other.0.id)
	}
}

impl RcSyncerState {
	pub async fn get_by_id(id: Uuid) -> Result<Option<Self>> {
		Ok(db::rcsyncer::Entity::find_by_id(id)
			.one(&*db::get())
			.await?
			.map(RcSyncerState))
	}

	pub async fn get(lang: &str) -> Result<Option<Self>> {
		Self::get_by_id(Page::get_lang_id(lang)).await
	}

	pub async fn get_or_init(lang: &str) -> Result<Self> {
		if let Some(page) = Self::get(lang).await? {
			Ok(page)
		} else {
			let new = db::rcsyncer::ActiveModel {
				id: ActiveValue::Set(Page::get_lang_id(lang)),
				last_synced_at: ActiveValue::Set(Utc::now().naive_utc()),
				last_rc_id: ActiveValue::Set(0),
			};
			Ok(Self(new.insert(&*db::get()).await?))
		}
	}

	pub fn id(&self) -> &Uuid {
		&self.0.id
	}

	pub fn last_synced_at(&self) -> DateTime<Utc> {
		self.0.last_synced_at.and_utc()
	}

	pub fn last_rc_id(&self) -> u32 {
		self.0.last_rc_id as u32
	}
}

pub async fn sync_rc(lang: &str) -> Result<()> {
	let app = App::get();
	let bot = app.mwbot(lang).await?;
	let state = RcSyncerState::get_or_init(lang).await?;

	let end_time = Utc::now();
	let mut gen = RecentChanges::new()
		.order(recent_changes::Order::Newer)
		.start(state.last_synced_at().to_owned())
		.end(end_time)
		.types(vec![
			"edit".to_string(),
			"new".to_string(),
			"log".to_string(),
		])
		.generate(&bot);
	let mut checked_pages = BTreeSet::new();
	let mut last_rcid = state.last_rc_id();
	while let Some(rc) = gen.recv().await {
		let rc = rc?;
		trace!(lang, rc = rc.rcid, "syncing RC");
		last_rcid = rc.rcid;
		let title = if let Some(title) = &rc.title {
			title
		} else {
			continue;
		};
		if checked_pages.contains(title) {
			continue;
		}

		if rc.type_ == "log" {
			if let Some(logtype) = rc.logtype {
				if logtype == "delete" {
					// sync delete
					if let Some(dbpage) = Page::get_by_name(lang, title).await? {
						dbpage.delete().await?;
					}
				}
				if logtype != "upload" {
					continue;
				}
			}
		}
		checked_pages.insert(title.to_owned());
		if let Some(dbpage) = Page::get_or_init(lang, title).await? {
			info!(rc = rc.rcid, page = title, "mark page for re-lint for RC");
			dbpage.mark_check().await?;
		}
	}

	let mut state = state.0.into_active_model();
	state.last_synced_at = ActiveValue::Set(end_time.naive_utc());
	state.last_rc_id = ActiveValue::Set(last_rcid as i32);
	state.update(&*db::get()).await?;

	Ok(())
}

pub async fn run_rc_syncer() {
	let app = App::get();

	loop {
		tokio::select! {
			_ = app.resync_pages_notify.notified()=>{},
			_ = tokio::time::sleep(std::time::Duration::from_secs(site::SYNC_RC_PEROID))=>{}
		}
		for lang in &site::SYNC_RC {
			if let Err(error) = sync_rc(lang).instrument(info_span!("sync_rc", lang)).await {
				error!(%error, lang, "failed to sync RC");
			}
		}
	}
}
