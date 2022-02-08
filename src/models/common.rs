use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnyType {
    LuaStr(Vec<u8>),
    Str(String),
    Integer(i32),
    Table(HashMap<String, AnyType>),
}
