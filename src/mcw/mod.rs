use phf::{phf_map, phf_set};
use uuid::{uuid, Uuid};

use crate::{
	checkers,
	linter::{checker::Checker, generic::incomplete_interlang::IncompleteInterlangLinkChecker},
};

pub const SITE_NAME: &str = "Minecraft Wiki";

pub fn get_wiki_url(lang: &str) -> String {
	if lang == "en" {
		"https://minecraft.wiki".to_string()
	} else if lang == "lzh" {
		"https://lzh-staging.minecraft.wiki".to_string()
	} else {
		format!("https://{}.minecraft.wiki", lang)
	}
}

pub const ROOT_UUID_NS: Uuid = uuid!("5dd8c71e-1bed-44a3-b4ec-05d08ad9c2ec");

pub const ALLOWED_NAMESPACES: phf::Map<&str, phf::Set<&str>> = phf_map! {
	"zh" => phf_set![
			"",
			"Minecraft Wiki",
			"File",
			"MediaWiki",
			"Template",
			"Help",
			"Category",
			"Module",
			"Gadget",
			"Gadget definition",
			"Minecraft Dungeons",
			"Minecraft Earth",
			"Minecraft Story Mode",
			"Minecraft Legends",
		],
	"en" => phf_set![
			"",
			"Minecraft Wiki",
			"File",
			"MediaWiki",
			"Template",
			"Help",
			"Category",
			"Module",
			"Gadget",
			"Gadget definition",
			"Minecraft Dungeons",
			"Minecraft Earth",
			"Minecraft Story Mode",
			"Minecraft Legends",
		],
};

pub const SYNC_ALL_PAGES_PEROID: u64 = 60 * 60 * 24;

pub const SYNC_RC: phf::Set<&str> = phf_set!["zh", "en"];
pub const SYNC_RC_PEROID: u64 = 30;

pub const SQLITE_INTERVAL_OPTIMIZE_PEROID: u64 = 60 * 60 * 24;

pub const LINTER_MAX_RETRIES: u32 = 5;
pub const LINTER_RETRY_DELAY: i64 = 10 * 60;

pub const I18N_FALLBACK_LANGUAGE: &str = "en_us";
pub const I18N_DEFAULT_LANGUAGE: &str = "en_us";

pub fn init_checkers() -> Vec<Checker> {
	checkers![IncompleteInterlangLink]
}
