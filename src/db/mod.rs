use std::{
    cell::LazyCell,
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use mlua::Lua;
use parking_lot::Mutex;

use self::info::CacheTypeRef;

pub mod info;

pub static LUA: Mutex<LazyCell<Lua>> = Mutex::new(LazyCell::new(Lua::new));
pub static REGISTRY: Mutex<BTreeMap<String, CacheTypeRef>> = Mutex::new(BTreeMap::new());

pub async fn check_path(path: &Path) -> Result<Option<(CacheTypeRef, PathBuf)>> {
    let cleaners = REGISTRY.lock();
    let name = &path
        .file_name()
        .ok_or_else(|| anyhow!("{} cant be resolved", path.display()))?
        .to_string_lossy()
        .to_string();
    if let Some(reference) = cleaners.get(name) {
        let lua = LUA.lock();
        let resolved = reference.resolve(&lua)?;
        if resolved.filter(path)? {
            return Ok(Some((reference.to_owned(), path.to_path_buf())));
        }
    }
    Ok(None)
}
