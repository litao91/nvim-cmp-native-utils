use mlua::prelude::*;
mod log;
#[mlua::lua_module]
fn libnvim_cmp_native_utils(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("log", log::make_log_tbl(lua)?)?;
    Ok(exports)
}
