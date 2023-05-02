use mlua::{Table, UserData, UserDataMethods};

use crate::db::{
    self,
    info::{CacheType, CacheTypeRef},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct RegistryAccess;

impl UserData for RegistryAccess {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("add", |lua, _, reference: CacheTypeRef| {
            let resolved = reference.resolve(lua).unwrap();
            db::REGISTRY
                .lock()
                .insert(resolved.get_file_name().unwrap(), reference);
            Ok(())
        });
        methods.add_method_mut("remove", |lua, _, reference: CacheTypeRef| {
            let resolved = reference.resolve(lua).unwrap();
            db::REGISTRY
                .lock()
                .remove(&resolved.get_file_name().unwrap());
            Ok(())
        });
        methods.add_method_mut("get_all", |_, _, ()| {
            Ok(db::REGISTRY.lock().keys().cloned().collect::<Vec<_>>())
        });
        methods.add_method_mut("create", |lua, _, table: Table| {
            let id = format!("REGISTRY_{}", table.get::<_, String>("id")?);
            let cache_type = CacheType::from((table.clone(), CacheTypeRef(id.clone())));
            lua.globals().set(id.clone(), table)?;
            db::REGISTRY
                .lock()
                .insert(cache_type.get_file_name().unwrap(), CacheTypeRef(id));
            Ok(())
        });
    }
}
