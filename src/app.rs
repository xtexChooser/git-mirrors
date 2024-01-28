use std::{
	collections::BTreeMap,
	sync::{Arc, OnceLock},
};

use anyhow::{bail, Result};
use parking_lot::RwLock;
use tokio::sync::Notify;

pub use mwbot::Bot as MwBot;
pub use mwbot::Page as MwPage;

use tracing::info;

use crate::db::DatabaseManager;

pub const USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"/",
	env!("CARGO_PKG_VERSION"),
	"(",
	env!("CARGO_PKG_REPOSITORY"),
	")"
);

pub struct App {
	pub linter_notify: Notify,
	pub bots: RwLock<BTreeMap<String, Arc<MwBot>>>,
	pub db: Arc<DatabaseManager>,
}

static GLOBAL_APP: OnceLock<Arc<App>> = OnceLock::new();

impl App {
	async fn new() -> Result<Arc<Self>> {
		Ok(Arc::new(Self {
			linter_notify: Notify::const_new(),
			bots: RwLock::new(BTreeMap::new()),
			db: Arc::new(DatabaseManager::new().await?),
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

	fn get_wiki_url(lang: &str) -> String {
		if lang == "en" {
			"https://minecraft.wiki".to_owned()
		} else {
			format!("https://{}.minecraft.wiki", lang)
		}
	}

	async fn new_bot(&self, lang: &str) -> Result<MwBot> {
		info!(lang, "Init mwbot");
		let mut builder = MwBot::builder(
			format!("{}/api.php", Self::get_wiki_url(lang)),
			format!("{}/rest.php/", Self::get_wiki_url(lang)),
		)
		.set_mark_as_bot(true)
		.set_respect_nobots(true)
		.set_user_agent(USER_AGENT.into());
		let username = std::env::var("SPOCK_WIKI_BOT_USERNAME")?;
		if let Ok(oauth_token) = std::env::var("SPOCK_WIKI_BOT_TOKEN") {
			builder = builder.set_oauth2_token(username, oauth_token);
		} else if let Ok(password) = std::env::var("SPOCK_WIKI_BOT_BOTPASSWD") {
			builder = builder.set_botpassword(username, password);
		} else {
			bail!("bot auth creds not found")
		}
		Ok(builder.build().await?)
	}

	pub async fn mwbot(&self, lang: &str) -> Result<Arc<MwBot>> {
		if let Some(bot) = self.bots.read().get(lang) {
			return Ok(bot.to_owned());
		}

		let mut bots = self.bots.write();
		if let Some(bot) = bots.get(lang) {
			Ok(bot.to_owned())
		} else {
			bots.insert(lang.to_string(), Arc::new(self.new_bot(lang).await?));
			Ok(bots.get(lang).unwrap().to_owned())
		}
	}
}
