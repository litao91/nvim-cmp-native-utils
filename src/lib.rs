use mlua::prelude::*;
mod log;
mod utils;
use crate::utils::matcher;
#[mlua::lua_module]

fn create_matcher_table(lua: &Lua) -> LuaResult<LuaTable> {
    let tbl = lua.create_table()?;
    tbl.set(
        "match",
        lua.create_function(
            |lua, (input, word, words): (LuaString, LuaString, LuaTable)| {
                let arg_input = input.as_bytes();
                let arg_word = word.as_bytes();
                let mut arg_words_val = Vec::new();
                for pair in words.pairs::<LuaValue, LuaString>() {
                    let (_, v) = pair?;
                    arg_words_val.push(v);
                }
                let arg_words: Vec<&[u8]> = arg_words_val.iter().map(|i| i.as_bytes()).collect();
                let matched = matcher::do_match(arg_input, arg_word, arg_words.as_ref());
                let r = lua.create_table()?;
                r.set(1, matched.0)?;
                let matches = lua.create_table()?;
                let mut count = 1;
                for m in matched.1 {
                    let m_lua = lua.create_table()?;
                    m_lua.set("input_match_start", m.input_match_start + 1)?;
                    m_lua.set("input_match_end", m.input_match_end)?;
                    m_lua.set("word_match_start", m.word_match_start + 1)?;
                    m_lua.set("word_match_end", m.word_match_end)?;
                    m_lua.set("index", m.index + 1)?;
                    m_lua.set("strict_ratio", m.strict_ratio)?;
                    m_lua.set("fuzzy", m.fuzzy)?;
                    matches.set(count, m_lua)?;
                    count += 1;
                }
                r.set(2, matches)?;
                Ok(r)
            },
        )?,
    )?;

    Ok(tbl)
}

#[mlua::lua_module]
fn libnvim_cmp_native_utils(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("log", log::make_log_tbl(lua)?)?;
    exports.set("matcher", create_matcher_table(lua)?)?;
    Ok(exports)
}
