use ::log::debug;
use fancy_regex::Regex;
use mlua::prelude::*;

use crate::{
    context::Context,
    models::lsp::{CompletionItem, InsertTextFormat, TextEdit},
    utils::{
        byte_char,
        matcher::{self, MatchRegion},
        misc, str_utils,
    },
};

pub struct Entry<'lua> {
    completion_item: CompletionItem,
    context: Context,
    source_offset: i32,
    offset: Option<i32>,
    word: Option<String>,
    pub entry: LuaTable<'lua>,
}

pub fn get_completion_item(tbl: &LuaTable, lua: &Lua) -> LuaResult<CompletionItem> {
    let resolved_completion_item: Option<LuaTable> = tbl.get("resolved_completion_item")?;
    let completion_item: LuaTable = tbl.get("completion_item")?;
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

impl<'lua> FromLua<'lua> for Entry<'lua> {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => {
                let completion_item = get_completion_item(&tbl, lua)?;
                Ok(Self {
                    completion_item,
                    context: tbl.get("context")?,
                    source_offset: tbl.get("source_offset")?,
                    offset: None,
                    word: None,
                    entry: tbl,
                })
            }
            LuaValue::Nil => Err(LuaError::FromLuaConversionError {
                from: "Nil",
                to: "Entry",
                message: None,
            }),
            LuaValue::Boolean(_) => Err(LuaError::FromLuaConversionError {
                from: "Boolean",
                to: "Entry",
                message: None,
            }),
            LuaValue::LightUserData(_) => Err(LuaError::FromLuaConversionError {
                from: "LightUserData",
                to: "Entry",
                message: None,
            }),
            LuaValue::Integer(_) => Err(LuaError::FromLuaConversionError {
                from: "Integer",
                to: "Entry",
                message: None,
            }),
            LuaValue::Number(_) => Err(LuaError::FromLuaConversionError {
                from: "Number",
                to: "Entry",
                message: None,
            }),
            LuaValue::String(_) => Err(LuaError::FromLuaConversionError {
                from: "String",
                to: "Entry",
                message: None,
            }),
            LuaValue::Function(_) => Err(LuaError::FromLuaConversionError {
                from: "Function",
                to: "Entry",
                message: None,
            }),
            LuaValue::Thread(_) => Err(LuaError::FromLuaConversionError {
                from: "Thread",
                to: "Entry",
                message: None,
            }),
            LuaValue::UserData(_) => Err(LuaError::FromLuaConversionError {
                from: "UserData",
                to: "Entry",
                message: None,
            }),
            LuaValue::Error(_) => Err(LuaError::FromLuaConversionError {
                from: "Error",
                to: "Entry",
                message: None,
            }),
        }
    }
}

impl<'lua> Entry<'lua> {
    pub fn get_filter_text(&self) -> &str {
        if let Some(filter_text) = &self.completion_item.filter_text {
            filter_text
        } else {
            self.completion_item.label.trim()
        }
    }
    pub fn get_override(&self) -> LuaResult<(i32, i32)> {
        if let Some(text_edit) = &self.completion_item.text_edit {
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
            let s = misc::to_vimindex(&self.context.cursor_line, r.start.character as usize) as i32;
            let e = misc::to_vimindex(&self.context.cursor_line, r.end.character as usize) as i32;
            let before = self.context.cursor.col as i32 - s;
            let after = e - self.context.cursor.col as i32;
            Ok((before, after))
        } else {
            Ok((0, 0))
        }
    }
    pub fn get_word(&mut self) -> LuaResult<String> {
        if let Some(word) = &self.word {
            return Ok(word.to_owned());
        }
        let mut word;

        if let Some(text_edit) = &self.completion_item.text_edit {
            word = text_edit.new_text.trim();
            let override_range = self.get_override()?;
            if 0 < override_range.1
                || self.completion_item.insert_text_format == InsertTextFormat::Snippet
            {
                word = str_utils::get_word(&word, 0);
            }
        } else {
            match &self.completion_item.insert_text {
                Some(lua_str) => {
                    word = std::str::from_utf8(lua_str.as_bytes())?.trim();
                    if self.completion_item.insert_text_format == InsertTextFormat::Snippet {
                        word = str_utils::get_word(&word, 0);
                    }
                }
                _ => {
                    word = &self.completion_item.label;
                }
            }
        }
        self.word = Some(word.to_owned());
        Ok(str_utils::oneline(word).to_owned())
    }
    pub fn get_offset(&mut self) -> LuaResult<i32> {
        if let Some(offset) = self.offset {
            return Ok(offset);
        }
        let mut offset = self.source_offset;
        if let Some(text_edit) = &self.completion_item.text_edit {
            let range = if let Some(range) = &text_edit.insert {
                Some(range)
            } else if let Some(range) = &text_edit.range {
                Some(range)
            } else {
                None
            };
            if let Some(range) = range {
                let c =
                    misc::to_vimindex(&self.context.cursor_line, range.start.character as usize);
                for idx in c..self.source_offset as usize {
                    if !byte_char::is_white(self.context.cursor_line.as_bytes()[idx]) {
                        offset = idx as i32;
                        break;
                    }
                }
            }
        } else {
            let word = self.get_word()?;
            ::log::debug!("word: {}, source_offset: {}", word, self.source_offset);
            for idx in (self.source_offset as usize+ 1 - word.len()..self.source_offset as usize).rev()
            {
                let c = self.context.cursor_line.as_bytes()[idx];
                if byte_char::is_white(c) {
                    break;
                }
                let mut matched = true;
                for i in 0..self.source_offset as usize - idx {
                    let c1 = word.as_bytes()[i];
                    let c2 = self.context.cursor_line.as_bytes()[idx + i - 1];
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
        self.offset = Some(offset);
        Ok(offset)
    }
    pub fn do_match(&mut self, input: &str) -> LuaResult<(f64, Vec<MatchRegion>)> {
        let filter_text = self.get_filter_text().to_owned();
        let word = self.get_word()?;
        let (mut score, mut matches) = matcher::do_match(
            input.as_bytes(),
            filter_text.as_bytes(),
            &[word.as_bytes(), self.completion_item.label.as_bytes()],
        );
        if score - 0.0 < 0.0001 {
            let offset = self.get_offset()?;
            if let Some(text_edit) = &self.completion_item.text_edit {
                let diff = self.source_offset - offset;
                if diff > 0 {
                    let prefix = &self.context.cursor_line.as_str().as_bytes()
                        [offset as usize..self.source_offset as usize];
                    let prefix_str = std::str::from_utf8(prefix)?;
                    let mut accept = false;

                    let prefix_pattern: Regex = Regex::new("^[^a-zA-Z]+$").unwrap();
                    accept = accept
                        || prefix_pattern
                            .is_match(prefix_str)
                            .map_err(|e| e.to_lua_err())?;
                    accept = accept || text_edit.new_text.find(prefix_str).is_some();
                    if accept {
                        let (s, m) = matcher::do_match(
                            input.as_bytes(),
                            &format!("{}{}", prefix_str, filter_text).as_bytes(),
                            &[word.as_bytes(), self.completion_item.label.as_bytes()],
                        );
                        score = s;
                        matches = m;
                    }
                    // TODO: implement this
                }
            }
        }
        if filter_text != self.completion_item.label.as_str() {
            let (_, m) = matcher::do_match(
                input.as_bytes(),
                self.completion_item.label.as_bytes(),
                &[word.as_bytes(), self.completion_item.label.as_bytes()],
            );
            matches = m;
        }
        let r = (score, matches);
        Ok(r)
    }
}
