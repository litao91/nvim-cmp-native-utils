use mlua::prelude::*;
use std::sync::Arc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ContextReason {
    Auto,
    Manual,
    TriggerOnly,
    None,
}

impl<'lua> FromLua<'lua> for ContextReason {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::String(s) => {
                let s_str = s.to_str()?;
                match s_str {
                    "auto" => Ok(ContextReason::Auto),
                    "manual" => Ok(ContextReason::Manual),
                    "triggerOnly" => Ok(ContextReason::TriggerOnly),
                    "none" => Ok(ContextReason::None),
                    _ => Err(LuaError::FromLuaConversionError {
                        from: "String",
                        to: "ContextReason",
                        message: Some(format!("Unknown ContextReason: {}", s_str)),
                    }),
                }
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "ContextReason",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ContextOption {
    pub reason: ContextReason,
}

impl<'lua> FromLua<'lua> for ContextOption {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => {
                let reason_value = tbl.get::<_, LuaValue>("reason")?;
                Ok(Self {
                    reason: match reason_value {
                        LuaValue::Nil => ContextReason::None,
                        _ => ContextReason::from_lua(reason_value, lua)?,
                    },
                })
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "ContextOption",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cursor {
    pub row: i32,
    pub col: i32,
    pub line: i32,
    pub character: i32,
}

impl<'lua> FromLua<'lua> for Cursor {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Self {
                row: tbl.get("row")?,
                col: tbl.get("col")?,
                line: tbl.get("line")?,
                character: tbl.get("character")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Cursor",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub id: usize,
    pub prev_context: Option<Arc<Context>>,
    pub option: ContextOption,
    pub filetype: Option<String>,
    pub time: u64,
    pub bufnr: i32,

    pub cursor_line: String,

    pub cursor: Cursor,

    pub cursor_before_line: String,
    pub cursor_after_line: String,
}

impl<'lua> FromLua<'lua> for Context {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Self {
                id: tbl.get("id")?,
                prev_context: {
                    let prev_context_v: LuaValue = tbl.get("prev_context")?;
                    match prev_context_v {
                        LuaValue::Nil => None,
                        _ => Some(Arc::new(Context::from_lua(prev_context_v, lua)?)),
                    }
                },
                option: ContextOption::from_lua(tbl.get::<_, LuaValue>("option")?, lua)?,
                filetype: {
                    let filetype_v: LuaValue = tbl.get("filetype")?;
                    match filetype_v {
                        LuaValue::Nil => None,
                        _ => Some(String::from_lua(filetype_v, lua)?),
                    }
                },
                time: tbl.get("time")?,
                bufnr: tbl.get("bufnr")?,
                cursor_line: tbl.get("cursor_line")?,
                cursor: Cursor::from_lua(tbl.get("cursor")?, lua)?,
                cursor_before_line: tbl.get("cursor_before_line")?,
                cursor_after_line: tbl.get("cursor_after_line")?,
            }),
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "Context",
                message: None,
            }),
        }
    }
}
