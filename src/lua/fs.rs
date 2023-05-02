use std::{
    fs,
    path::{self, PathBuf},
};

use mlua::{prelude::LuaError, FromLua, ToLua, UserData, UserDataFields, UserDataMethods};
use owo_colors::{colors::css::DarkGrey, OwoColorize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct LuaPath(PathBuf);

impl From<PathBuf> for LuaPath {
    fn from(value: PathBuf) -> Self {
        LuaPath(value)
    }
}
impl Into<PathBuf> for LuaPath {
    fn into(self) -> PathBuf {
        self.0
    }
}
impl<'lua> FromLua<'lua> for LuaPath {
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        Ok(LuaPath(String::from_lua(lua_value, lua)?.into()))
    }
}
impl<'lua> ToLua<'lua> for LuaPath {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(self
            .0
            .to_str()
            .ok_or(mlua::Error::SafetyError(
                "non-unicode chars in path".to_string(),
            ))?
            .to_lua(lua)?)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct FsAccess;

impl UserData for FsAccess {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("separator", |_, _| Ok(path::MAIN_SEPARATOR_STR));
    }

    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("absolute", |_, _, path: LuaPath| {
            Ok(LuaPath::from(path::absolute(path.0)?))
        });
        methods.add_method_mut("is_file", |_, _, path: LuaPath| Ok(path.0.is_file()));
        methods.add_method_mut("is_dir", |_, _, path: LuaPath| Ok(path.0.is_dir()));
        methods.add_method_mut("is_symlink", |_, _, path: LuaPath| Ok(path.0.is_symlink()));
        methods.add_method_mut("is_absoulte", |_, _, path: LuaPath| {
            Ok(path.0.is_absolute())
        });
        methods.add_method_mut("exists", |_, _, path: LuaPath| Ok(path.0.exists()));
        methods.add_method_mut("get_name", |_, _, path: LuaPath| {
            Ok(path
                .0
                .file_name()
                .ok_or(LuaError::RuntimeError("no file name".to_string()))?
                .to_str()
                .ok_or(mlua::Error::SafetyError(
                    "non-unicode chars in file name".to_string(),
                ))?
                .to_owned())
        });
        methods.add_method_mut("get_ext", |_, _, path: LuaPath| {
            Ok(path
                .0
                .extension()
                .ok_or(LuaError::RuntimeError("no extension".to_string()))?
                .to_str()
                .ok_or(mlua::Error::SafetyError(
                    "non-unicode chars in name extension".to_string(),
                ))?
                .to_owned())
        });
        methods.add_method_mut("starts_with", |_, _, (path1, path2): (LuaPath, LuaPath)| {
            Ok(path1.0.starts_with(path2.0))
        });
        methods.add_method_mut("ends_with", |_, _, (path1, path2): (LuaPath, LuaPath)| {
            Ok(path1.0.ends_with(path2.0))
        });
        methods.add_method_mut("parent", |_, _, path: LuaPath| {
            Ok(LuaPath(
                path.0
                    .parent()
                    .ok_or(LuaError::RuntimeError("no parent dir".to_string()))?
                    .to_path_buf(),
            ))
        });
        methods.add_method_mut("rmrf", |_, _, path: LuaPath| {
            println!(
                "{}",
                format!("Remove (rf) {}", path.0.display()).fg::<DarkGrey>()
            );
            Ok(fs::remove_dir_all(path.0)?)
        });
        methods.add_method_mut("rmd", |_, _, path: LuaPath| {
            println!(
                "{}",
                format!("Remove (d ) {}", path.0.display()).fg::<DarkGrey>()
            );
            Ok(fs::remove_dir(path.0)?)
        });
        methods.add_method_mut("rm", |_, _, path: LuaPath| {
            println!(
                "{}",
                format!("Remove (  ) {}", path.0.display()).fg::<DarkGrey>()
            );
            Ok(fs::remove_file(path.0)?)
        });
        methods.add_method_mut("side", |_, _, (path, file): (LuaPath, LuaPath)| {
            let mut buf = path
                .0
                .parent()
                .ok_or(LuaError::RuntimeError("no parent dir".to_string()))?
                .to_path_buf();
            buf.push(file.0);
            Ok(LuaPath(buf))
        });
    }
}
