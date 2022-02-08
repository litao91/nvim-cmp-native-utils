use mlua::prelude::*;

use crate::{models::lsp::TextEdit, utils::{misc, byte_char}};

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

pub fn get_word<'lua>(lua: &Lua, entry: &LuaTable<'lua>, completion_item: &LuaTable<'lua>, text_edit: &Option<TextEdit>) -> LuaResult<&'lua [u8]> {
        let mut word;
        if let Some(text_edit) = text_edit {
            word = text_edit.new_text.trim();
            let override_range = self.get_override();
            if 0 < override_range.1
                || self.completion_item.insert_text_format == InsertTextFormat::Snippet
            {
                word = str_utils::get_word(&word, 0);
            }
        } else {
            if let Some(insert_text) = &self.completion_item.insert_text {
                word = insert_text.trim();
                if self.completion_item.insert_text_format == InsertTextFormat::Snippet {
                    word = str_utils::get_word(&word, 0);
                }
            } else {
                word = self.completion_item.label.trim();
            }
        }
        let r = str_utils::oneline(word).to_owned();
        r
}

pub fn get_offset<'lua>(lua: &Lua, entry: &LuaTable<'lua>) -> LuaResult<i32> {
    let offset = entry.get::<_, i32>("source_offset")?;
    let completion_item = get_completion_item(entry)?;
    let text_edit_lua: LuaValue = completion_item.get("textEdit")?;
    let text_edit = match text_edit_lua {
        LuaValue::Nil => None,
        _ => Some(TextEdit::from_lua(text_edit_lua, lua)?)
    };
    let cursor_line = entry.get::<_, LuaTable>("context")?.get::<_, LuaString>("cursor_line")?.as_bytes();
    let cursor_line_str = std::str::from_utf8(cursor_line)?;
    if let Some(text_edit) = text_edit {
            let range = if let Some(range) = &text_edit.insert {
                Some(range)
            } else if let Some(range) = &text_edit.range {
                Some(range)
            } else {
                None
            };
            if let Some(range) = range {
                let c = misc::to_vimindex(cursor_line_str, range.start.character);
                for idx in c..entry.get::<_, usize>("source_offset")?{
                    if !byte_char::is_white(cursor_line[idx]) {
                        offset = idx as i32;
                        break;
                    }
                }
            }
    } else {
    }
    Ok(offset)

}
