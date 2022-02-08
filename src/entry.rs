use mlua::prelude::*;

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

pub fn get_offset<'lua>(lua: &Lua, entry: &LuaTable<'lua>) -> LuaResult<i32> {
    let offset = entry.get::<_, i32>("source_offset")?;
    let completion_item = get_completion_item(entry)?;
    let text_edit: LuaValue = completion_item.get("textEdit");
    todo!()
}
