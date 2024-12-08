use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use mlua::{FromLua, Function, Lua, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(transparent)]
pub struct CacheTypeRef(pub String);

impl CacheTypeRef {
    pub fn resolve<'a>(&self, lua: &'a Lua) -> Result<CacheType> {
        Ok((lua.globals().get::<Table>(self.0.as_str())?, self.clone()).into())
    }
}

impl FromLua for CacheTypeRef {
    #[inline]
    fn from_lua(lua_value: mlua::Value, lua: &Lua) -> mlua::Result<Self> {
        Ok(String::from_lua(lua_value, lua)?.into())
    }
}

impl From<String> for CacheTypeRef {
    fn from(value: String) -> Self {
        CacheTypeRef(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CacheType(Table, CacheTypeRef);

impl From<(Table, CacheTypeRef)> for CacheType {
    fn from(value: (Table, CacheTypeRef)) -> Self {
        Self(value.0, value.1)
    }
}

impl CacheType {
    pub fn to_raw_table(&self) -> &Table {
        &self.0
    }

    pub fn to_ref(&self) -> &CacheTypeRef {
        &self.1
    }

    pub fn get_name(&self) -> Result<String> {
        Ok(self.0.get::<String>("name")?)
    }

    pub fn get_file_name(&self) -> Result<String> {
        Ok(self.0.get::<String>("file_name")?)
    }

    pub fn get_default_checked(&self) -> Result<bool> {
        if !self.0.contains_key("default_selected")? {
            return Ok(true);
        }
        Ok(self.0.get::<bool>("default_selected")?)
    }

    pub fn filter(&self, path: &Path) -> Result<bool> {
        if !self.0.contains_key("filter")? {
            return Ok(true);
        }
        Ok(self
            .0
            .get::<Function>("filter")?
            .call::<bool>(path.to_string_lossy().to_string())?)
    }

    pub fn to_display(&self, path: &Path) -> Result<PathBuf> {
        if !self.0.contains_key("to_display")? {
            return Ok(path
                .parent()
                .ok_or(anyhow!("got a root dir with cache"))?
                .to_path_buf());
        }
        Ok(self
            .0
            .get::<Function>("to_display")?
            .call::<String>(path.to_string_lossy().to_string())?
            .into())
    }

    pub fn clean(&self, path: &Path) -> Result<()> {
        if !self.0.contains_key("do_clean")? {
            return self.fast_clean(path);
        }
        Ok(self
            .0
            .get::<Function>("do_clean")?
            .call::<_>(path.to_string_lossy().to_string())?)
    }

    pub fn fast_clean(&self, path: &Path) -> Result<()> {
        Ok(self
            .0
            .get::<Function>("do_fast_clean")?
            .call::<_>(path.to_string_lossy().to_string())?)
    }
}
