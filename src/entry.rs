use mlua::prelude::*;

use crate::{
    context::Context,
    models::lsp::{CompletionItem, InsertTextFormat, TextEdit},
    utils::{byte_char, misc, str_utils},
};

pub fn get_filter_text(completion_item: &LuaTable) {}

pub fn get_completion_item<'lua>(lua: &Lua, entry: &LuaTable<'lua>) -> LuaResult<CompletionItem> {
    let resolved_completion_item: Option<LuaTable> = entry.get("resolved_completion_item")?;
    let completion_item: LuaTable = entry.get("completion_item")?;
    let item_lua = if let Some(resolved_completion_item) = resolved_completion_item {
        for pair in resolved_completion_item.pairs::<LuaValue, LuaValue>() {
            let (k, v) = pair?;
            if v != LuaValue::Nil {
                completion_item.set(k, v)?;
            }
        }
        completion_item
    } else {
        completion_item
    };
    CompletionItem::from_lua(LuaValue::Table(item_lua), lua)
}

pub static INSERT_TEXT_FORMAT_SNIPPET: i32 = 2;

pub fn get_override(
    entry: &LuaTable,
    text_edit: &TextEdit,
    ctx: &Context,
) -> LuaResult<(i32, i32)> {
    let r = if let Some(insert) = &text_edit.insert {
        Some(insert.clone())
    } else {
        if let Some(range) = &text_edit.range {
            Some(range.clone())
        } else {
            None
        }
    }
    .unwrap();
    let s = misc::to_vimindex(&ctx.cursor_line, r.start.character as usize) as i32;
    let e = misc::to_vimindex(&ctx.cursor_line, r.end.character as usize) as i32;
    let col = entry
        .get::<_, LuaTable>("context")?
        .get::<_, LuaTable>("cursor")?
        .get::<_, i32>("col")?;
    let before = col - s;
    let after = e - col;
    Ok((before, after))
}

pub fn get_word(
    entry: &LuaTable,
    completion_item: &CompletionItem,
    text_edit: &Option<TextEdit>,
    ctx: &Context,
) -> LuaResult<String> {
    let mut word;

    if let Some(text_edit) = text_edit {
        word = text_edit.new_text.trim();
        let override_range = get_override(entry, text_edit, &ctx)?;
        if 0 < override_range.1 || completion_item.insert_text_format == InsertTextFormat::Snippet {
            word = str_utils::get_word(&word, 0);
        }
    } else {
        match &completion_item.insert_text {
            Some(lua_str) => {
                word = std::str::from_utf8(lua_str.as_bytes())?.trim();
                if completion_item.insert_text_format == InsertTextFormat::Snippet {
                    word = str_utils::get_word(word, 0);
                }
            }
            _ => {
                word = &completion_item.label;
            }
        }
    }
    Ok(str_utils::oneline(word).to_owned())
}

pub fn get_offset(lua: &Lua, ctx: &Context, entry: &LuaTable) -> LuaResult<i32> {
    let source_offset = entry.get::<_, i32>("source_offset")?;
    let mut offset = source_offset;
    let completion_item = get_completion_item(lua, entry)?;
    if let Some(text_edit) = completion_item.text_edit {
        let range = if let Some(range) = &text_edit.insert {
            Some(range)
        } else if let Some(range) = &text_edit.range {
            Some(range)
        } else {
            None
        };
        if let Some(range) = range {
            let c = misc::to_vimindex(&ctx.cursor_line, range.start.character as usize);
            for idx in c..source_offset as usize {
                if !byte_char::is_white(ctx.cursor_line.as_bytes()[idx]) {
                    offset = idx as i32;
                    break;
                }
            }
        }
    } else {
        let word = get_word(entry, &completion_item, &completion_item.text_edit, ctx)?;
        for idx in (source_offset as usize - word.len()..source_offset as usize).rev() {
            let c = ctx.cursor_line.as_bytes()[idx];
            if byte_char::is_white(c) {
                break;
            }
            let mut matched = true;
            for i in 0..source_offset as usize - idx {
                let c1 = word.as_bytes()[i];
                let c2 = ctx.cursor_line.as_bytes()[idx + i - 1];
                if (c1 == 0) || (c2 == 0) || (c1 != c2) {
                    matched = false;
                    break;
                }
            }
            if matched {
                offset = if offset < idx as i32 {
                    offset
                } else {
                    idx as i32
                }
            }
        }
    }
    Ok(offset)
}
