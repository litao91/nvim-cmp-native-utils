use std::collections::HashMap;

use mlua::prelude::*;

use crate::{context::Context, entry, utils::matcher};

pub fn get_entries<'a>(lua: &'a Lua, source: &LuaTable, ctx: &Context) -> LuaResult<LuaTable<'a>> {
    let r = lua.create_table()?;
    if source.get::<_, i32>("offset")? == -1 {
        return Ok(r);
    }
    let target_entries: LuaTable = source.get("entries")?;
    let target_len = target_entries.raw_len();
    let mut inputs = HashMap::<i32, &str>::new();
    for i in 1..(target_len + 1) {
        let e_lua = target_entries.get::<_, LuaTable>(i)?;
        let o = entry::get_offset(lua, ctx, &e_lua)?;
        inputs
            .entry(o)
            .or_insert(&ctx.cursor_before_line.as_str()[o as usize..]);
    }
    Ok(r)
}
