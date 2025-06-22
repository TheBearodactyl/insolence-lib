use mlua::prelude::*;

pub(crate) fn capitalize(lua: &Lua, word: Option<String>) -> LuaResult<LuaValue> {
    match word {
        None => Ok(LuaValue::Nil),
        Some(word) if word.is_empty() => Ok(LuaValue::String(lua.create_string("")?)),
        Some(word) => {
            let mut chars = word.chars();
            let first_char = chars.next().unwrap().to_uppercase().collect::<String>();
            let rest = chars.collect::<String>().to_lowercase();

            Ok(LuaValue::String(
                lua.create_string(format!("{}{}", first_char, rest))?,
            ))
        }
    }
}
