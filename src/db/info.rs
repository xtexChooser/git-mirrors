use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use mlua::{FromLua, Function, Lua, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(transparent)]
pub struct CacheTypeRef(pub String);

impl CacheTypeRef {
    pub fn resolve<'a>(&self, lua: &'a Lua) -> Result<CacheType<'a>> {
        /*Ok((
            lua.load(&self.0)
                .set_name(format!("resolver for cache type {}", &self.0))?
                .eval::<Table>()?,
            self.clone(),
        )
            .into())*/
        Ok((
            lua.globals().get::<_, Table>(self.0.as_str())?,
            self.clone(),
        )
            .into())
    }
}

impl<'lua> FromLua<'lua> for CacheTypeRef {
    #[inline]
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        Ok(String::from_lua(lua_value, lua)?.into())
    }
}

impl From<String> for CacheTypeRef {
    fn from(value: String) -> Self {
        CacheTypeRef(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CacheType<'a>(Table<'a>, CacheTypeRef);

impl<'a> From<(Table<'a>, CacheTypeRef)> for CacheType<'a> {
    fn from(value: (Table<'a>, CacheTypeRef)) -> Self {
        Self(value.0, value.1)
    }
}

impl<'a> CacheType<'a> {
    pub fn to_raw_table(&self) -> &Table<'a> {
        &self.0
    }

    pub fn to_ref(&self) -> &CacheTypeRef {
        &self.1
    }

    pub fn get_name(&self) -> Result<String> {
        Ok(self.0.get::<_, String>("name")?)
    }

    pub fn get_file_name(&self) -> Result<String> {
        Ok(self.0.get::<_, String>("file_name")?)
    }

    pub fn get_default_checked(&self) -> Result<bool> {
        if !self.0.contains_key("default_selected")? {
            return Ok(true);
        }
        Ok(self.0.get::<_, bool>("default_selected")?)
    }

    pub fn filter(&self, path: &PathBuf) -> Result<bool> {
        if !self.0.contains_key("filter")? {
            return Ok(true);
        }
        Ok(self
            .0
            .get::<_, Function>("filter")?
            .call::<_, bool>(path.to_string_lossy().to_string())?)
    }

    pub fn to_display(&self, path: &PathBuf) -> Result<PathBuf> {
        if !self.0.contains_key("to_display")? {
            return Ok(path
                .parent()
                .ok_or(anyhow!("got a root dir with cache"))?
                .to_path_buf());
        }
        Ok(self
            .0
            .get::<_, Function>("to_display")?
            .call::<_, String>(path.to_string_lossy().to_string())?
            .into())
    }

    pub fn clean(&self, path: &PathBuf) -> Result<()> {
        if !self.0.contains_key("do_clean")? {
            return self.fast_clean(path);
        }
        Ok(self
            .0
            .get::<_, Function>("do_clean")?
            .call::<_, _>(path.to_string_lossy().to_string())?)
    }

    pub fn fast_clean(&self, path: &PathBuf) -> Result<()> {
        if !self.0.contains_key("do_fast_clean")? {
            return fs::remove_dir_all(path).map_err(anyhow::Error::from);
        }
        Ok(self
            .0
            .get::<_, Function>("do_fast_clean")?
            .call::<_, _>(path.to_string_lossy().to_string())?)
    }
}
