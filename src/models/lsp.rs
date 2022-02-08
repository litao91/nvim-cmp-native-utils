use mlua::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InsertTextFormat {
    PlainText,
    Snippet,
}

impl<'lua> FromLua<'lua> for InsertTextFormat {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Integer(i) => match i {
                1 => Ok(InsertTextFormat::PlainText),
                2 => Ok(InsertTextFormat::Snippet),
                _ => Err(LuaError::FromLuaConversionError {
                    from: "Integer",
                    to: "InsertTextFormat",
                    message: Some(format!("Unknown value: {}", i)),
                }),
            },
            _ => Err(LuaError::FromLuaConversionError {
                from: lua_value.type_name(),
                to: "InsertTextFormat",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InsertTextMode {
    AsIs,
    AdjustIndentation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
    pub deprecated: bool,
    pub preselect: bool,
    pub sort_text: String,
    pub filter_text: Option<String>,
    pub insert_text: Option<String>,
    pub insert_text_format: InsertTextFormat,
    pub text_edit: Option<TextEdit>,
    pub word: String,
    pub dup: Option<bool>,
}

impl<'lua> FromLua<'lua> for CompletionItem {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Self {
                label: tbl.get("label")?,
                detail: tbl.get("detail")?,
                deprecated: tbl.get("deprecated")?,
                preselect: tbl.get("preselect")?,
                sort_text: tbl.get("sortText")?,
                filter_text: tbl.get("filterText")?,
                insert_text: tbl.get("insertText")?,
                insert_text_format: tbl.get("insertTextFormat")?,
                text_edit: tbl.get("textEdit")?,
                word: tbl.get("wrod")?,
                dup: tbl.get("dup")?,
            }),
            LuaValue::Nil => Err(LuaError::FromLuaConversionError {
                from: "Nil",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::Boolean(_) => Err(LuaError::FromLuaConversionError {
                from: "Boolean",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::LightUserData(_) => Err(LuaError::FromLuaConversionError {
                from: "LightUserData",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::Integer(_) => Err(LuaError::FromLuaConversionError {
                from: "Integer",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::Number(_) => Err(LuaError::FromLuaConversionError {
                from: "Number",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::String(_) => Err(LuaError::FromLuaConversionError {
                from: "String",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::Function(_) => Err(LuaError::FromLuaConversionError {
                from: "Function",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::Thread(_) => Err(LuaError::FromLuaConversionError {
                from: "Thread",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::UserData(_) => Err(LuaError::FromLuaConversionError {
                from: "UserData",
                to: "CompletionItem",
                message: None,
            }),
            LuaValue::Error(_) => Err(LuaError::FromLuaConversionError {
                from: "Error",
                to: "CompletionItem",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub line: i32,
    pub character: i32,
}

impl<'lua> FromLua<'lua> for Position {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Self {
                line: tbl.get("line")?,
                character: tbl.get("character")?,
            }),
            LuaValue::Nil => Err(LuaError::FromLuaConversionError {
                from: "Nil",
                to: "Position",
                message: None,
            }),
            LuaValue::Boolean(_) => Err(LuaError::FromLuaConversionError {
                from: "Boolean",
                to: "Position",
                message: None,
            }),
            LuaValue::LightUserData(_) => Err(LuaError::FromLuaConversionError {
                from: "LightUserData",
                to: "Position",
                message: None,
            }),
            LuaValue::Integer(_) => Err(LuaError::FromLuaConversionError {
                from: "Integer",
                to: "Position",
                message: None,
            }),
            LuaValue::Number(_) => Err(LuaError::FromLuaConversionError {
                from: "Number",
                to: "Position",
                message: None,
            }),
            LuaValue::String(_) => Err(LuaError::FromLuaConversionError {
                from: "String",
                to: "Position",
                message: None,
            }),
            LuaValue::Function(_) => Err(LuaError::FromLuaConversionError {
                from: "Function",
                to: "Position",
                message: None,
            }),
            LuaValue::Thread(_) => Err(LuaError::FromLuaConversionError {
                from: "Thread",
                to: "Position",
                message: None,
            }),
            LuaValue::UserData(_) => Err(LuaError::FromLuaConversionError {
                from: "UserData",
                to: "Position",
                message: None,
            }),
            LuaValue::Error(_) => Err(LuaError::FromLuaConversionError {
                from: "Error",
                to: "Position",
                message: None,
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl<'lua> FromLua<'lua> for Range {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Self {
                start: Position::from_lua(tbl.get("start")?, lua)?,
                end: Position::from_lua(tbl.get("start")?, lua)?,
            }),
            LuaValue::Nil => Err(LuaError::FromLuaConversionError {
                from: "Nil",
                to: "Range",
                message: None,
            }),
            LuaValue::Boolean(_) => Err(LuaError::FromLuaConversionError {
                from: "Boolean",
                to: "Range",
                message: None,
            }),
            LuaValue::LightUserData(_) => Err(LuaError::FromLuaConversionError {
                from: "LightUserData",
                to: "Range",
                message: None,
            }),
            LuaValue::Integer(_) => Err(LuaError::FromLuaConversionError {
                from: "Integer",
                to: "Range",
                message: None,
            }),
            LuaValue::Number(_) => Err(LuaError::FromLuaConversionError {
                from: "Number",
                to: "Range",
                message: None,
            }),
            LuaValue::String(_) => Err(LuaError::FromLuaConversionError {
                from: "String",
                to: "Range",
                message: None,
            }),
            LuaValue::Function(_) => Err(LuaError::FromLuaConversionError {
                from: "Function",
                to: "Range",
                message: None,
            }),
            LuaValue::Thread(_) => Err(LuaError::FromLuaConversionError {
                from: "Thread",
                to: "Range",
                message: None,
            }),
            LuaValue::UserData(_) => Err(LuaError::FromLuaConversionError {
                from: "UserData",
                to: "Range",
                message: None,
            }),
            LuaValue::Error(_) => Err(LuaError::FromLuaConversionError {
                from: "Error",
                to: "Range",
                message: None,
            }),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TextEdit {
    pub range: Option<Range>,
    pub insert: Option<Range>,  // for InsertReplace
    pub replace: Option<Range>, // for InsertReplace
    pub new_text: String,
    // pub new_text: Vec<u8>,
}

impl<'lua> FromLua<'lua> for TextEdit {
    fn from_lua(lua_value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Table(tbl) => Ok(Self {
                range: {
                    let v: LuaValue = tbl.get("range")?;
                    match v {
                        LuaValue::Nil => None,
                        _ => Some(Range::from_lua(v, lua)?),
                    }
                },
                insert: {
                    let v: LuaValue = tbl.get("insert")?;
                    match v {
                        LuaValue::Nil => None,
                        _ => Some(Range::from_lua(v, lua)?),
                    }
                },
                replace: {
                    let v: LuaValue = tbl.get("replace")?;
                    match v {
                        LuaValue::Nil => None,
                        _ => Some(Range::from_lua(v, lua)?),
                    }
                },
                new_text: tbl.get::<_, String>("newText")?,
            }),
            LuaValue::Nil => Err(LuaError::FromLuaConversionError {
                from: "Nil",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::Boolean(_) => Err(LuaError::FromLuaConversionError {
                from: "Boolean",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::LightUserData(_) => Err(LuaError::FromLuaConversionError {
                from: "LightUserData",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::Integer(_) => Err(LuaError::FromLuaConversionError {
                from: "Integer",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::Number(_) => Err(LuaError::FromLuaConversionError {
                from: "Number",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::String(_) => Err(LuaError::FromLuaConversionError {
                from: "String",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::Function(_) => Err(LuaError::FromLuaConversionError {
                from: "Function",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::Thread(_) => Err(LuaError::FromLuaConversionError {
                from: "Thread",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::UserData(_) => Err(LuaError::FromLuaConversionError {
                from: "UserData",
                to: "TextEdit",
                message: None,
            }),
            LuaValue::Error(_) => Err(LuaError::FromLuaConversionError {
                from: "Error",
                to: "TextEdit",
                message: None,
            }),
        }
    }
}
