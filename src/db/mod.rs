use std::{cell::LazyCell, collections::BTreeMap, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Result};
use mlua::Lua;
use parking_lot::Mutex;

use crate::builtin;

use self::info::CacheTypeRef;

pub mod info;

pub static LUA: Mutex<LazyCell<Lua>> = Mutex::new(LazyCell::new(Lua::new));
pub static REGISTRY: Mutex<BTreeMap<String, CacheTypeRef>> = Mutex::new(BTreeMap::new());

pub async fn init_lua() -> Result<()> {
    let lua = LUA.lock();
    // init globals
    let globals = lua.globals();
    globals.set(
        "register_type",
        lua.create_function(|lua, reference: CacheTypeRef| {
            let resolved = reference.resolve(lua).unwrap();
            REGISTRY
                .lock()
                .insert(resolved.get_file_name().unwrap(), reference);
            Ok(())
        })?,
    )?;
    globals.set(
        "unregister_type",
        lua.create_function(|lua, reference: CacheTypeRef| {
            let resolved = reference.resolve(lua).unwrap();
            REGISTRY.lock().remove(&resolved.get_file_name().unwrap());
            Ok(())
        })?,
    )?;
    // load scripts
    lua.load(builtin::MAIN_LUA)
        .set_name("builtin main.lua")?
        .exec()?;
    let script_dirs = vec![
        PathBuf::from_str("/usr/share/build-clean.d")?,
        PathBuf::from_str("/usr/lib/build-clean.d")?,
        PathBuf::from_str("/usr/local/build-clean.d")?,
        PathBuf::from_str("build-clean.d")?,
        PathBuf::from_str(".build-clean.d")?,
        PathBuf::from_str(&std::env::var("BUILD_CLEAN_RC").unwrap_or_else(|_| {
            std::env::var("HOME").unwrap_or("/opt".to_string()) + "/build-clean.d"
        }))?,
    ];
    let mut script_files = vec![];
    for dir in script_dirs {
        if dir.exists() && dir.is_dir() {
            for entry in dir.read_dir()? {
                let entry = entry?;
                if entry.file_type()?.is_file()
                    && entry.file_name().to_string_lossy().ends_with(".lua")
                {
                    script_files.push(entry.path());
                }
            }
        }
    }
    for file in script_files {
        lua.load(&file).exec()?;
    }

    Ok(())
}

pub async fn check_path(path: &PathBuf) -> Result<Option<(CacheTypeRef, PathBuf)>> {
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
            return Ok(Some((reference.to_owned(), path.clone())));
        }
    }
    Ok(None)
}
