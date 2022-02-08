use mlua::prelude::*;
use serde::ser::{Serialize, Serializer};
use serde_json;
pub fn to_vimindex(text: &str, utfindex: usize) -> usize {
    let mut r = 0;
    let mut chars = text.chars();
    for i in 0..utfindex {
        if let Some(c) = chars.next() {
            r += c.len_utf8();
        }
    }
    return r + 1;
}

pub fn to_utfindex(text: &str, mut vim_index: usize) -> usize {
    let mut utfindex = 0;
    for c in text.chars() {
        vim_index -= c.len_utf8();
        utfindex += 1;
        if vim_index <= 0 {
            break;
        }
    }

    utfindex
}

pub fn inspect(lua: &Lua, lua_value: &LuaValue) -> LuaResult<String> {
    let mut writer = Vec::with_capacity(128);
    let mut serializer = serde_json::Serializer::new(&mut writer);
    match lua_value {
        LuaValue::Nil => serializer.serialize_unit(),
        LuaValue::Boolean(b) => serializer.serialize_bool(*b),
        #[allow(clippy::useless_conversion)]
        LuaValue::Integer(i) => serializer.serialize_i64((*i).into()),
        #[allow(clippy::useless_conversion)]
        LuaValue::Number(n) => serializer.serialize_f64((*n).into()),
        LuaValue::String(s) => s.serialize(&mut serializer),
        LuaValue::Table(t) => match t.serialize(&mut serializer) {
            Ok(o) => Ok(o),
            Err(_) => Ok(()),
        },
        LuaValue::UserData(ud) => ud.serialize(&mut serializer),
        LuaValue::LightUserData(ud) if ud.0.is_null() => serializer.serialize_none(),
        LuaValue::Error(_)
        | LuaValue::LightUserData(_)
        | LuaValue::Function(_)
        | LuaValue::Thread(_) => Ok(()),
    }
    .unwrap();

    Ok(String::from_utf8(writer).unwrap())
}
