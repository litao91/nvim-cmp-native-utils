use mlua::prelude::*;

pub fn get_entries<'a>(
    lua: &'a Lua,
    source_table: &LuaTable,
    ctd: &LuaTable,
) -> LuaResult<LuaTable<'a>> {
    let r = lua.create_table()?;
    if source_table.get::<_, i32>("offset")? == -1 {
        return Ok(r);
    }
    let target_entries: LuaTable = source_table.get("entries")?;
    let target_len = target_entries.raw_len();
    for i in 1..(target_len + 1) {
    }
    Ok(r)
}
