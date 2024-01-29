use phf::{phf_map, phf_set};

pub fn get_wiki_url(lang: &str) -> String {
	if lang == "en" {
		"https://minecraft.wiki".to_owned()
	} else {
		format!("https://{}.minecraft.wiki", lang)
	}
}

pub const SYNC_ALL_PAGES_NAMESPACES: phf::Map<&str, phf::Set<&str>> = phf_map! {
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
