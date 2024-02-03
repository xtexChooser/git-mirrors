use std::{
	collections::BTreeMap,
	num::NonZeroUsize,
	sync::{Arc, OnceLock},
};

use anyhow::{bail, Result};
use lru::LruCache;
use parking_lot::RwLock;
use tokio::sync::Notify;

pub use mwbot::Bot as MwBot;
pub use mwbot::Page as MwPage;

use tracing::info;

use crate::{config, db::DatabaseManager, linter::LinterState, site, web};

pub const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"/",
	env!("CARGO_PKG_VERSION"),
	"(",
	env!("CARGO_PKG_REPOSITORY"),
	")"
);

pub struct App {
	pub bots: RwLock<BTreeMap<String, MwBot>>,
	pub db: Arc<DatabaseManager>,
	pub resync_pages_notify: Notify,
	pub login_lru: RwLock<LruCache<String, web::auth::AuthResult>>,
	pub linter: Arc<LinterState>,
}

static GLOBAL_APP: OnceLock<Arc<App>> = OnceLock::new();

config!(WIKI_BOT_USERNAME, str, required);
config!(WIKI_BOT_TOKEN, str, optional);
config!(WIKI_BOT_BOTPASSWD, str, optional);

impl App {
	async fn new() -> Result<Arc<Self>> {
		Ok(Arc::new(Self {
			bots: RwLock::new(BTreeMap::new()),
			db: Arc::new(DatabaseManager::new().await?),
			resync_pages_notify: Notify::new(),
			login_lru: RwLock::new(LruCache::new(
				NonZeroUsize::new(30).unwrap(),
			)),
			linter: Arc::new(LinterState::new()?),
		}))
	}

	pub async fn init() -> Result<()> {
		if GLOBAL_APP.set(Self::new().await?).is_err() {
			bail!("App is already inited")
		}
		Ok(())
	}

	pub fn get() -> Arc<Self> {
		GLOBAL_APP.get().expect("App is not initialized").to_owned()
	}

	async fn new_bot(&self, lang: &str) -> Result<MwBot> {
		info!(lang, "Init mwbot");
		let mut builder = MwBot::builder(
			format!("{}/api.php", site::get_wiki_url(lang)),
			format!("{}/rest.php/", site::get_wiki_url(lang)),
		);
		builder = builder
			.set_mark_as_bot(true)
			.set_respect_nobots(true)
			.set_user_agent(USER_AGENT.into());

		let username = *CONFIG_WIKI_BOT_USERNAME;
		if let Some(oauth_token) = *CONFIG_WIKI_BOT_TOKEN {
			builder = builder.set_oauth2_token(
				username.to_string(),
				oauth_token.to_string(),
			);
		} else if let Some(password) = *CONFIG_WIKI_BOT_BOTPASSWD {
			builder = builder
				.set_botpassword(username.to_string(), password.to_string());
		} else {
			bail!("bot auth creds not found")
		}

		Ok(builder.build().await?)
	}

	#[allow(clippy::await_holding_lock)]
	pub async fn mwbot(&self, lang: &str) -> Result<MwBot> {
		if let Some(bot) = self.bots.read().get(lang) {
			return Ok(bot.clone());
		}

		let mut bots = self.bots.write();
		if let Some(bot) = bots.get(lang) {
			Ok(bot.clone())
		} else {
			bots.insert(lang.to_string(), self.new_bot(lang).await?);
			Ok(bots.get(lang).unwrap().clone())
		}
	}
}
