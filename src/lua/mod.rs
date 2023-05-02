use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use mlua::Lua;

use crate::builtin;

use self::{fs::FsAccess, registry::RegistryAccess};

pub mod fs;
pub mod registry;

pub fn init_lua(lua: &Lua) -> Result<()> {
    // init globals
    let globals = lua.globals();
    globals.set("registry", RegistryAccess)?;
    globals.set("fs", FsAccess)?;

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
