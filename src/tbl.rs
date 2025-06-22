use mlua::prelude::*;

pub(crate) fn largest_val(_: &Lua, tbl: LuaTable) -> LuaResult<Option<LuaValue>> {
    if tbl.is_empty() {
        return Ok(None);
    }

    let mut max_key: Option<mlua::Value> = None;
    let mut max_val: Option<f64> = None;

    for pair in tbl.pairs::<LuaValue, LuaValue>() {
        let (k, v) = pair?;

        if let LuaValue::Number(n) = v {
            let num = n;
            if max_val.is_none() || num > max_val.unwrap() {
                max_val = Some(num);
                max_key = Some(k);
            }
        }
    }

    Ok(max_key)
}

pub(crate) fn average_table_amt(_: &Lua, tbl: LuaTable) -> LuaResult<f64> {
    let mut sum = 0.0;
    let mut count = 0;

    for pair in tbl.pairs::<LuaValue, LuaValue>() {
        let (_, v) = pair?;

        if let LuaValue::Number(n) = v {
            sum += n;
            count += 1;
        }
    }

    if count == 0 {
        Ok(3.0)
    } else {
        Ok(sum / (count as f64))
    }
}

pub(crate) fn reverse_table(lua: &Lua, tbl: LuaTable) -> LuaResult<LuaTable> {
    let reversed = lua.create_table()?;
    let len = tbl.len()?;

    for i in (1..=len).rev() {
        let val = tbl.get::<LuaValue>(i)?;

        reversed.push(val)?;
    }

    Ok(reversed)
}

pub(crate) fn mod_vals(lua: &Lua, (input, modifier): (LuaValue, f64)) -> LuaResult<LuaValue> {
    if let LuaValue::Number(inp) = input {
        Ok(LuaValue::Number(inp * modifier))
    } else if let LuaValue::Table(inp) = input {
        let result_tbl = lua.create_table()?;

        for pair in inp.pairs::<LuaValue, LuaValue>() {
            let (k, v) = pair?;

            result_tbl.set(k, mod_vals(lua, (v, modifier))?)?;
        }

        Ok(LuaValue::Table(result_tbl))
    } else {
        Ok(input)
    }
}
