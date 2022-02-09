use context::Context;
use mlua::prelude::*;
mod context;
mod entry;
mod log;
mod models;
mod source;
mod utils;
use crate::utils::matcher;
use std::time::{SystemTime, UNIX_EPOCH};
use ::log::debug;

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

    tbl.set(
        "bench_rs",
        lua.create_function(|lua, ()| {
            for _ in 0..10000 {
                matcher::do_match(b"", b"a", &Vec::new());
                matcher::do_match(b"a", b"a", &Vec::new());
                matcher::do_match(b"ab", b"a", &Vec::new());
                matcher::do_match(b"ab", b"ab", &Vec::new());
                matcher::do_match(b"ab", b"a_b", &Vec::new());
                matcher::do_match(b"ab", b"a_b_c", &Vec::new());
                matcher::do_match(b"ac", b"a_b_c", &Vec::new());

                matcher::do_match(b"bora", b"border-radius", &Vec::new());
                matcher::do_match(b"woroff", b"word_offset", &Vec::new());
                matcher::do_match(b"call", b"call", &Vec::new());
                matcher::do_match(b"call", b"condition_all", &Vec::new());
                matcher::do_match(b"Buffer", b"Buffer", &Vec::new());
                matcher::do_match(b"Buffer", b"buffer", &Vec::new());
                matcher::do_match(b"fmodify", b"fnamemodify", &Vec::new());
                matcher::do_match(b"candlesingle", b"candle#accept#single", &Vec::new());
                matcher::do_match(b"conso", b"console", &Vec::new());
                matcher::do_match(b"conso", b"ConstantSourceNode", &Vec::new());
                matcher::do_match(b"var_", b"var_dump", &Vec::new());
                matcher::do_match(b"my_", b"my_awesome_variable", &Vec::new());
                matcher::do_match(b"my_", b"completion_matching_strategy_list", &Vec::new());
                matcher::do_match(b"luacon", b"lua_context", &Vec::new());
                matcher::do_match(b"luacon", b"LuaContext", &Vec::new());
                matcher::do_match(b"call", b"calc", &Vec::new());

                matcher::do_match(b"vi", b"void#", &Vec::new());
                matcher::do_match(b"vo", b"void#", &Vec::new());
                matcher::do_match(b"usela", b"useLayoutEffect", &Vec::new());
                matcher::do_match(b"usela", b"useDataLayer", &Vec::new());
                matcher::do_match(b"true", b"v:true", vec![b"true".as_slice()].as_slice());
                matcher::do_match(b"true", b"true", &Vec::new());
                matcher::do_match(b"g", b"get", vec![b"get".as_slice()].as_slice());
                matcher::do_match(b"g", b"dein#get", vec![b"dein#get".as_slice()].as_slice());
                matcher::do_match(b"2", b"[[2021", &Vec::new());
            }

            Ok(())
        })?,
    )?;

    Ok(tbl)
}

#[mlua::lua_module]
fn libnvim_cmp_native_utils(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("log", log::make_log_tbl(lua)?)?;
    exports.set("matcher", create_matcher_table(lua)?)?;
    exports.set(
        "timestamp",
        lua.create_function(|_, ()| {
            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_millis();
            Ok(since_the_epoch)
        })?,
    )?;
    exports.set(
        "get_entries_from_source",
        lua.create_function(
            |lua, (source, ctx, limit): (LuaTable, LuaValue, i64)| -> LuaResult<LuaTable> {
                let ctx = &Context::from_lua(ctx, lua)?;
                // debug!("ctx: {:?}", ctx);
                let r = source::get_entries(lua, &source, ctx, limit)?;
                ::log::debug!("processed {} entries", r.len()?);
                Ok(r)
            },
        )?,
    )?;
    Ok(exports)
}
