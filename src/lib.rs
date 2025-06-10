#![allow(clippy::only_used_in_recursion)]

use mlua::prelude::*;
use rand::{Rng, seq::SliceRandom};

fn between(_: &Lua, (num, min, max): (f64, f64, f64)) -> LuaResult<bool> {
    Ok(num >= min && num <= max)
}

fn largest_val(_: &Lua, tbl: LuaTable) -> LuaResult<Option<mlua::Value>> {
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

fn average_table_amt(_: &Lua, tbl: LuaTable) -> LuaResult<f64> {
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

fn reverse_table(lua: &Lua, tbl: LuaTable) -> LuaResult<LuaTable> {
    let reversed = lua.create_table()?;
    let len = tbl.len()?;

    for i in (1..=len).rev() {
        let val = tbl.get::<LuaValue>(i)?;

        reversed.push(val)?;
    }

    Ok(reversed)
}

fn capitalize(lua: &Lua, word: Option<String>) -> LuaResult<LuaValue> {
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

fn every_day_im_shufflin(lua: &Lua, tbl: LuaValue) -> LuaResult<LuaValue> {
    let table = match tbl {
        LuaValue::Table(t) => t,
        _ => return Ok(tbl),
    };

    let mut values = Vec::new();
    let mut paths = Vec::new();

    collect_values(lua, &table, "", &mut values, &mut paths)?;

    let mut rng = rand::rng();
    values.shuffle(&mut rng);

    let mut value_index = 0;
    let result = reconstruct(lua, &table, "", &values, &mut value_index)?;

    Ok(result)
}

fn collect_values(
    lua: &Lua,
    table: &LuaTable,
    path: &str,
    values: &mut Vec<LuaValue>,
    paths: &mut Vec<String>,
) -> LuaResult<()> {
    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (k, v) = pair?;
        let new_path = format!("{}.{}", path, k.to_string()?);

        match v {
            LuaValue::Table(t) => collect_values(lua, &t, &new_path, values, paths)?,
            _ => {
                values.push(v);
                paths.push(new_path);
            }
        }
    }

    Ok(())
}

fn reconstruct(
    lua: &Lua,
    table: &LuaTable,
    path: &str,
    values: &[LuaValue],
    value_index: &mut usize,
) -> LuaResult<LuaValue> {
    let new_table = lua.create_table()?;

    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (k, v) = pair?;
        let new_path = format!("{}.{}", path, k.to_string()?);

        match v {
            LuaValue::Table(t) => {
                let reconstructed = reconstruct(lua, &t, &new_path, values, value_index)?;
                new_table.set(k, reconstructed)?;
            }
            _ => {
                new_table.set(k, LuaValue::Nil)?;
            }
        }
    }

    for pair in table.pairs::<LuaValue, LuaValue>() {
        let (k, v) = pair?;

        if !matches!(v, LuaValue::Table(_)) {
            if *value_index < values.len() {
                new_table.set(k, values[*value_index].clone())?;
                *value_index += 1;
            } else {
                new_table.set(k, LuaValue::Nil)?;
            }
        }
    }

    if new_table.len()? == 0 && *value_index < values.len() {
        Ok(values[*value_index].clone())
    } else {
        Ok(LuaValue::Table(new_table))
    }
}

/// Returns true if 64bit, false if 32bit
fn check_ptr_width() -> bool {
    #[cfg(target_pointer_width = "64")]
    {
        true
    }

    #[cfg(target_pointer_width = "32")]
    {
        false
    }
}

fn rand_mem_addr(lua: &Lua, _: ()) -> LuaResult<LuaValue> {
    let hex_digits = if check_ptr_width() { 16 } else { 8 };
    let mut rng = rand::rng();
    let mut addr = String::from("0x");

    for _ in 0..hex_digits {
        let n: u8 = rng.random_range(0..16);
        addr.push_str(&format!("{:X}", n));
    }

    Ok(LuaValue::String(lua.create_string(&addr)?))
}

fn placeholder_sprite(lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    let spr_loc = lua.create_table()?;

    spr_loc.set("x", 19)?;
    spr_loc.set("y", 0)?;

    Ok(spr_loc)
}

fn within(_: &Lua, (x, y): (f64, f64)) -> LuaResult<bool> {
    Ok((x - y).abs() <= x)
}

fn random_str(_: &Lua, (len, char_set): (u32, Option<String>)) -> LuaResult<String> {
    let chr_set = char_set
        .unwrap_or("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%^".into());
    let mut rand_str = String::new();

    for _ in 1..=len {
        let rand_idx = rand::rng().random_range(1..=chr_set.len());
        rand_str.push(chr_set.as_bytes()[rand_idx - 1] as char);
    }

    Ok(rand_str)
}

fn hex(lua: &Lua, hex: String) -> LuaResult<LuaTable> {
    let hex = if hex.len() <= 6 {
        format!("{}FF", hex)
    } else {
        hex
    };

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);

    let col = lua.create_table()?;
    col.set(1, r as f32 / 255.0)?;
    col.set(2, g as f32 / 255.0)?;
    col.set(3, b as f32 / 255.0)?;
    col.set(4, a as f32 / 255.0)?;

    Ok(col)
}

fn rand_hex_code(lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    let mut rng = rand::rng();
    let r = rng.random_range(0..=255);
    let g = rng.random_range(0..=255);
    let b = rng.random_range(0..=255);

    let hex_code = format!("{:02X}{:02X}{:02X}", r, g, b);

    hex(lua, hex_code)
}

fn rand_int(_: &Lua, (min, max): (i64, i64)) -> LuaResult<i64> {
    if min > max {
        println!("Min greater than max, assuming min is being used as max");
        Ok(rand::rng().random_range(max..min))
    } else {
        Ok(rand::rng().random_range(min..max))
    }
}

fn rand_num(_: &Lua, (min, max): (f64, f64)) -> LuaResult<f64> {
    if min > max {
        println!("Min greater than max, assuming min is being used as max");
        Ok(rand::rng().random_range(max..min))
    } else {
        Ok(rand::rng().random_range(min..max))
    }
}

fn random_table_of_strs(lua: &Lua, (str_len, tbl_len): (u32, u32)) -> LuaResult<LuaTable> {
    let str_tbl = lua.create_table()?;

    for _ in 0..=tbl_len {
        let rand_str = random_str(lua, (str_len, None))?;
        str_tbl.push(lua.create_string(rand_str.as_bytes())?)?;
    }

    Ok(str_tbl)
}

fn wave_number(_: &Lua, num: f64) -> LuaResult<f64> {
    if num == 0.0 {
        Ok(1.0)
    } else if num > 0.0 {
        Ok(-num - 1.0)
    } else {
        Ok(-num + 1.0)
    }
}

fn clamp(_: &Lua, (num, min, max): (f64, f64, f64)) -> LuaResult<f64> {
    if num < min {
        Ok(min)
    } else if num > max {
        Ok(max)
    } else {
        Ok(num)
    }
}

fn chance(_: &Lua, percent_chance: f32) -> LuaResult<bool> {
    Ok(percent_chance <= rand::rng().random_range(0.0..=100.0))
}

macro_rules! def_module {
    ($mod_name:ident, funcs [$($func_name:ident),*]) => {
        #[mlua::lua_module]
        fn $mod_name(lua: &Lua) -> LuaResult<LuaTable> {
            let exports = lua.create_table()?;
            $(
                exports.set(stringify!($func_name), lua.create_function($func_name)?)?;
            )*
            Ok(exports)
        }
    };
}

def_module!(
    insolence,
    funcs [
        between,
        largest_val,
        average_table_amt,
        reverse_table,
        capitalize,
        every_day_im_shufflin,
        random_str,
        rand_hex_code,
        rand_int,
        rand_num,
        wave_number,
        clamp,
        chance,
        within,
        random_table_of_strs,
        rand_mem_addr,
        placeholder_sprite
    ]
);
