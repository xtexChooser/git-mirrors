use std::collections::{BTreeMap, HashMap};

use anyhow::Result;
use phf::phf_map;

use crate::site;

static mut LANG: BTreeMap<(&str, &str), &str> = BTreeMap::new();
const LANG_DATA: phf::Map<&str, &str> = phf_map! {
	"en_us" => include_str!("../../langs/en_us.json"),
};

pub fn init() -> Result<()> {
	for (lang, json) in &LANG_DATA {
		let lang = *lang;
		let json = serde_json::from_str::<HashMap<String, String>>(*json)?;
		for (key, value) in json {
			unsafe {
				LANG.insert((lang, key.leak()), value.leak());
			}
		}
	}
	Ok(())
}

pub fn get(lang: &str, key: &'static str) -> &'static str {
	unsafe {
		LANG.get(&(lang, key))
			.or_else(|| LANG.get(&(site::I18N_FALLBACK_LANGUAGE, key)))
			.unwrap_or(&key)
	}
}
