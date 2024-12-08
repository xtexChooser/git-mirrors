use mlua::{Table, UserData, UserDataMethods};

use crate::db::{
    self,
    info::{CacheType, CacheTypeRef},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct RegistryAccess;

impl UserData for RegistryAccess {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("add", |lua, _, reference: CacheTypeRef| {
            let resolved = reference.resolve(lua).unwrap();
            db::REGISTRY
                .write()
                .insert(resolved.get_file_name().unwrap(), reference);
            Ok(())
        });
        methods.add_method_mut("remove", |lua, _, reference: CacheTypeRef| {
            let resolved = reference.resolve(lua).unwrap();
            db::REGISTRY
                .write()
                .remove(&resolved.get_file_name().unwrap());
            Ok(())
        });
        methods.add_method_mut("get_all", |_, _, ()| {
            Ok(db::REGISTRY.write().keys().cloned().collect::<Vec<_>>())
        });
        methods.add_method_mut("create", |lua, _, table: Table| {
            let id = format!("REGISTRY_{}", table.get::<String>("id")?);
            let cache_type = CacheType::from((table.clone(), CacheTypeRef(id.clone())));
            lua.globals().set(id.clone(), table)?;
            db::REGISTRY
                .write()
                .insert(cache_type.get_file_name().unwrap(), CacheTypeRef(id));
            Ok(())
        });
        methods.add_method_mut("add_ignore_dir_name", |_, _, name: String| {
            db::IGNORE_DIR_NAMES.write().push(name);
            Ok(())
        });
    }
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("ignore_dir_names", |_, _| {
            Ok(db::IGNORE_DIR_NAMES.write().clone())
        });
    }
}
