use mlua::prelude::*;

use crate::{
    models::lsp::TextEdit,
    utils::{byte_char, misc, str_utils},
};

pub fn get_completion_item<'lua>(entry: &LuaTable<'lua>) -> LuaResult<LuaTable<'lua>> {
    let resolved_completion_item: Option<LuaTable> = entry.get("resolved_completion_item")?;
    let completion_item: LuaTable = entry.get("completion_item")?;
    if let Some(resolved_completion_item) = resolved_completion_item {
        for pair in resolved_completion_item.pairs::<LuaValue, LuaValue>() {
            let (k, v) = pair?;
            if v != LuaValue::Nil {
                completion_item.set(k, v)?;
            }
        }
        Ok(completion_item)
    } else {
        Ok(completion_item)
    }
}

pub static INSERT_TEXT_FORMAT_SNIPPET: i32 = 2;

pub fn get_override(
    entry: &LuaTable,
    text_edit: &TextEdit,
    cursor_line: &str,
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
    let s = misc::to_vimindex(&cursor_line, r.start.character as usize) as i32;
    let e = misc::to_vimindex(&cursor_line, r.end.character as usize) as i32;
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
    completion_item: &LuaTable,
    text_edit: &Option<TextEdit>,
    cursor_line: &str,
) -> LuaResult<String> {
    let mut word;
    let label_lua = completion_item.get::<_, LuaString>("label")?;
    let insert_text_format = completion_item.get::<_, i32>("insertTextFormat")?;
    let insert_text_lua = completion_item.get::<_, LuaValue>("insertText")?;
    let insert_text = match &insert_text_lua {
        LuaValue::String(lua_str) => Some(lua_str),
        _ => None,
    };

    if let Some(text_edit) = text_edit {
        word = text_edit.new_text.trim();
        let override_range = get_override(entry, text_edit, cursor_line)?;
        if 0 < override_range.1 || insert_text_format == INSERT_TEXT_FORMAT_SNIPPET {
            word = str_utils::get_word(&word, 0);
        }
    } else {
        match insert_text {
            Some(lua_str) => {
                word = std::str::from_utf8(lua_str.as_bytes())?.trim();
                if insert_text_format == INSERT_TEXT_FORMAT_SNIPPET {
                    word = str_utils::get_word(word, 0);
                }
            }
            _ => {
                word = std::str::from_utf8(label_lua.as_bytes())?.trim();
            }
        }
    }
    Ok(str_utils::oneline(word).to_owned())
}

pub fn get_offset<'lua>(lua: &Lua, entry: &LuaTable<'lua>) -> LuaResult<i32> {
    let mut offset = entry.get::<_, i32>("source_offset")?;
    let completion_item = get_completion_item(entry)?;
    let text_edit_lua: LuaValue = completion_item.get("textEdit")?;
    let text_edit = match text_edit_lua {
        LuaValue::Nil => None,
        _ => Some(TextEdit::from_lua(text_edit_lua, lua)?),
    };
    let cursor_line = entry
        .get::<_, LuaTable>("context")?
        .get::<_, String>("cursor_line")?;
    if let Some(text_edit) = text_edit {
        let range = if let Some(range) = &text_edit.insert {
            Some(range)
        } else if let Some(range) = &text_edit.range {
            Some(range)
        } else {
            None
        };
        if let Some(range) = range {
            let c = misc::to_vimindex(&cursor_line, range.start.character as usize);
            for idx in c..entry.get::<_, usize>("source_offset")? {
                if !byte_char::is_white(cursor_line.as_bytes()[idx]) {
                    offset = idx as i32;
                    break;
                }
            }
        }
    } else {
    }
    Ok(offset)
}
