use std::collections::HashMap;

use mlua::prelude::*;

use crate::{context::Context, entry::Entry};

pub fn get_entries<'a>(
    lua: &'a Lua,
    source: &LuaTable,
    ctx: &Context,
    limit: i64,
) -> LuaResult<LuaTable<'a>> {
    let target_entries: LuaTable = source.get("entries")?;
    let target_len = target_entries.raw_len();
    let mut inputs = HashMap::<i32, &str>::new();
    let entries = lua.create_table()?;
    let mut num_entry = 1i32;
    for i in 1..(std::cmp::min(target_len, limit) + 1) {
        let e_lua = target_entries.get::<_, LuaValue>(i)?;
        let mut e = Entry::from_lua(e_lua, lua)?;
        let o = e.get_offset()?;
        let input = inputs
            .entry(o)
            .or_insert(&ctx.cursor_before_line.as_str()[o as usize..]);
        let matched = e.do_match(input)?;
        let score = matched.0;
        e.entry.set("score", score)?;
        e.entry.set("exact", false)?;
        if score >= 1.0 {
            let matches = lua.create_table()?;
            let mut count: i32 = 1;
            for m in matched.1 {
                let m_lua: LuaTable = m.to_lua(lua)?;
                matches.set(count, m_lua)?;
                count += 1;
            }
            e.entry.set("matches", matches)?;
            let eq_filter_text = &e.get_filter_text() == input;
            let eq_word = &e.get_word()? == input;
            e.entry.set("exact", eq_filter_text || eq_word)?;
            entries.set(num_entry, e.entry)?;
            num_entry += 1;
        }
    }
    Ok(entries)
}
