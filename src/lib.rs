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

fn rand_table_of_hex_codes(lua: &Lua, len: u32) -> LuaResult<LuaTable> {
    let col_table = lua.create_table()?;

    for _ in 0..=len {
        let rand_hex_code = rand_hex_code(lua, ())?;
        col_table.push(rand_hex_code)?;
    }

    Ok(col_table)
}

fn mod_vals(lua: &Lua, (input, modifier): (LuaValue, f64)) -> LuaResult<LuaValue> {
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
    rand::rng().reseed().expect("Failed to reseed");

    Ok(percent_chance <= rand::rng().random_range(0.0..=100.0))
}

fn exponentiate(lua: &Lua, (mut base, mut power): (f64, f64)) -> LuaResult<f64> {
    if power == 0.0 {
        return Ok(1.0);
    } else if power == 1.0 {
        return Ok(base);
    } else if power < 0.0 {
        return Ok(1.0 / exponentiate(lua, (base, -power))?);
    }

    let mut result = 1.0;

    while power > 0.0 {
        if power % 2.0 == 1.0 {
            result *= base;
        }

        base *= base;
        power = (power / 2.0).floor();
    }

    Ok(result)
}

fn boobs_sprite(lua: &Lua, mod_cfg: LuaValue) -> LuaResult<LuaTable> {
    let sprite_tbl = lua.create_table()?;

    if let LuaValue::Table(modcfg) = mod_cfg {
        let adult_mode = modcfg
            .get::<LuaTable>("config")?
            .get::<LuaValue>("adult_mode")?
            .as_boolean()
            .unwrap();

        if !adult_mode {
            sprite_tbl.set("x", 1000)?;
            sprite_tbl.set("y", 1000)?;
        } else if adult_mode {
            sprite_tbl.set("x", 12)?;
            sprite_tbl.set("y", 2)?;
        } else {
            sprite_tbl.set("x", 1000)?;
            sprite_tbl.set("y", 1000)?;
        }
    }

    Ok(sprite_tbl)
}

fn include_content(lua: &Lua, (name, ty): (String, String)) -> LuaResult<()> {
    let path = format!("src/content/{}/{}.lua", ty, name);

    let smods: LuaTable = lua.globals().get("SMODS")?;
    let load_file: LuaFunction = smods.get("load_file")?;
    let func: LuaFunction = load_file.call::<LuaFunction>(path)?;

    func.call::<LuaFunction>(())?;

    Ok(())
}

fn include(lua: &Lua, path: String) -> LuaResult<()> {
    let smods: LuaTable = lua.globals().get("SMODS")?;
    let load_file: LuaFunction = smods.get("load_file")?;
    let func: LuaFunction = load_file.call::<LuaFunction>(path)?;

    func.call::<LuaFunction>(())?;

    Ok(())
}

fn word_to_color(_: &Lua, word: String) -> LuaResult<String> {
    let mut hex_components: Vec<String> = Vec::new();

    for ch in word.chars() {
        let ascii_val = ch as u8;
        let hex = format!("{:02x}", ascii_val);

        hex_components.push(hex.chars().next().unwrap().to_string());
    }

    while hex_components.len() < 3 {
        if let Some(last) = hex_components.last().cloned() {
            hex_components.push(last);
        }
    }

    let hex_col = hex_components.concat();

    Ok(hex_col)
}

type RandJokerParams = (
    Option<String>,
    Option<LuaTable>,
    Option<String>,
    Option<LuaTable>,
    Option<bool>,
);

fn random_joker(
    lua: &Lua,
    (seed, excluded_flags, banned_card, pool, no_undiscovered): RandJokerParams,
) -> LuaResult<LuaValue> {
    // Default excluded flags if not provided
    let excluded_flags = excluded_flags.unwrap_or_else(|| {
        lua.create_table_from(vec![(1, "hidden"), (2, "no_doe"), (3, "no_grc")])
            .unwrap()
    });

    let mut selection = lua.create_table()?;
    selection.set("key", "n/a")?;

    #[allow(unused_assignments)]
    let mut passes = 0;
    let mut tries = 500;

    let pseudoseed: mlua::Function = lua.globals().get("pseudoseed")?;
    let pseudorandom_element: mlua::Function = lua.globals().get("pseudorandom_element")?;
    let g_table: LuaTable = lua.globals().get("G")?;
    let p_centers: LuaTable = g_table.get("P_CENTERS")?;

    // Get the pool table or default to G.P_CENTER_POOLS.Joker
    let pool_table = if let Some(pool) = pool {
        pool
    } else {
        let p_center_pools: LuaTable = g_table.get("P_CENTER_POOLS")?;
        p_center_pools.get("Joker")?
    };

    loop {
        tries -= 1;
        passes = 0;

        // Call pseudorandom_element
        let random_element: LuaTable = pseudorandom_element.call((
            pool_table.clone(),
            pseudoseed.call::<LuaValue>(seed.as_deref().unwrap_or("grc"))?,
        ))?;

        let key: String = random_element.get("key")?;
        selection = p_centers.get(key.clone())?;

        // Check discovered status
        let discovered: bool = selection.get("discovered").unwrap_or(false);
        if discovered || !no_undiscovered.unwrap_or(false) {
            // Check banned card
            if banned_card.is_none()
                || (banned_card.is_some() && banned_card.as_ref().unwrap() != &key)
            {
                passes += 1;
            }
        }

        // Check exit conditions
        let excluded_flags_len = excluded_flags.len()?;
        if passes >= excluded_flags_len || tries <= 0 {
            if tries <= 0 && no_undiscovered.unwrap_or(false) {
                return p_centers.get::<LuaValue>("c_strength");
            } else {
                return Ok(LuaValue::Table(selection));
            }
        }
    }
}

fn is_rigged_cryptid(lua: &Lua, card: LuaTable) -> LuaResult<bool> {
    let can_load = lua
        .globals()
        .get::<Option<LuaTable>>("SMODS")?
        .and_then(|smods| smods.get::<Option<LuaTable>>("Mods").ok().flatten())
        .and_then(|mods| mods.get::<Option<LuaTable>>("Cryptid").ok().flatten())
        .and_then(|mods| mods.get::<Option<bool>>("can_load").ok().flatten())
        .unwrap_or(false);

    if !can_load {
        return Ok(false);
    }

    let cry_rigged = card
        .get::<Option<LuaTable>>("ability")?
        .and_then(|ability| ability.get::<Option<bool>>("cry_rigged").ok().flatten())
        .unwrap_or(false);

    Ok(cry_rigged)
}

fn mod_cond(
    lua: &Lua,
    (mod_id, if_exists, otherwise): (String, LuaValue, LuaValue),
) -> LuaResult<LuaValue> {
    let can_load = lua
        .globals()
        .get::<Option<LuaTable>>("SMODS")?
        .and_then(|smods| smods.get::<Option<LuaTable>>("Mods").ok().flatten())
        .and_then(|mods| mods.get::<Option<LuaTable>>(mod_id).ok().flatten())
        .and_then(|mod_entry| mod_entry.get::<Option<bool>>("can_load").ok().flatten())
        .unwrap_or(false);

    Ok(if can_load { if_exists } else { otherwise })
}

fn count_num_of_joker(lua: &Lua, (prefix, key): (String, String)) -> LuaResult<u32> {
    let mut num_of_joker = 0;
    let target_name = format!("j_{}_{}", prefix, key);
    let cards = lua
        .globals()
        .get::<Option<LuaTable>>("G")?
        .and_then(|g| g.get::<Option<LuaTable>>("jokers").ok().flatten())
        .and_then(|jokers| jokers.get::<Option<LuaTable>>("cards").ok().flatten());

    if let Some(card_table) = cards {
        for pair in card_table.pairs::<LuaValue, LuaTable>().flatten() {
            let (_, card) = pair;
            let name_matches = card
                .get::<Option<LuaTable>>("ability")
                .ok()
                .flatten()
                .and_then(|ability| ability.get::<Option<String>>("name").ok().flatten())
                .map(|name| name == target_name)
                .unwrap_or(false);

            if name_matches {
                num_of_joker += 1;
            }
        }
    }

    Ok(num_of_joker)
}

fn register_items(lua: &Lua, (items, path): (Vec<String>, String)) -> LuaResult<()> {
    let smods: LuaTable = lua.globals().get("SMODS")?;
    let load_file: LuaFunction = smods.get("load_file")?;

    for item in items {
        let full_path = format!("{}/{}.lua", path, item);
        let result: (LuaValue, LuaValue) = load_file.call(full_path)?;

        let (load_func, err_str) = result;

        if let LuaValue::String(err) = err_str {
            let err_msg = err.to_string_lossy();
            return Err(mlua::Error::RuntimeError(format!(
                "[INSOLENCE] Error: {}",
                err_msg.as_str()
            )));
        }

        if let LuaValue::Function(func) = load_func {
            func.call::<LuaValue>(())?;
        }
    }

    Ok(())
}

#[mlua::lua_module]
fn libinsolence(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set(stringify!(between), lua.create_function(between)?)?;
    exports.set(stringify!(boobs_sprite), lua.create_function(boobs_sprite)?)?;
    exports.set(stringify!(largest_val), lua.create_function(largest_val)?)?;
    exports.set(
        stringify!(average_table_amt),
        lua.create_function(average_table_amt)?,
    )?;
    exports.set(
        stringify!(reverse_table),
        lua.create_function(reverse_table)?,
    )?;
    exports.set(stringify!(capitalize), lua.create_function(capitalize)?)?;
    exports.set(
        stringify!(every_day_im_shufflin),
        lua.create_function(every_day_im_shufflin)?,
    )?;
    exports.set(stringify!(mod_vals), lua.create_function(mod_vals)?)?;
    exports.set(stringify!(random_str), lua.create_function(random_str)?)?;
    exports.set(
        stringify!(rand_hex_code),
        lua.create_function(rand_hex_code)?,
    )?;
    exports.set(stringify!(exponentiate), lua.create_function(exponentiate)?)?;
    exports.set(stringify!(rand_int), lua.create_function(rand_int)?)?;
    exports.set(stringify!(rand_num), lua.create_function(rand_num)?)?;
    exports.set(stringify!(wave_number), lua.create_function(wave_number)?)?;
    exports.set(stringify!(clamp), lua.create_function(clamp)?)?;
    exports.set(stringify!(chance), lua.create_function(chance)?)?;
    exports.set(stringify!(within), lua.create_function(within)?)?;
    exports.set(
        stringify!(random_table_of_strs),
        lua.create_function(random_table_of_strs)?,
    )?;
    exports.set(
        stringify!(rand_mem_addr),
        lua.create_function(rand_mem_addr)?,
    )?;
    exports.set(
        stringify!(placeholder_sprite),
        lua.create_function(placeholder_sprite)?,
    )?;
    exports.set(
        stringify!(include_content),
        lua.create_function(include_content)?,
    )?;
    exports.set(stringify!(include), lua.create_function(include)?)?;
    exports.set(
        stringify!(word_to_color),
        lua.create_function(word_to_color)?,
    )?;
    exports.set(stringify!(random_joker), lua.create_function(random_joker)?)?;
    exports.set(
        stringify!(rand_table_of_hex_codes),
        lua.create_function(rand_table_of_hex_codes)?,
    )?;
    exports.set(
        stringify!(is_rigged_cryptid),
        lua.create_function(is_rigged_cryptid)?,
    )?;
    exports.set(stringify!(mod_cond), lua.create_function(mod_cond)?)?;
    exports.set(
        "count_num_of_joker",
        lua.create_function(count_num_of_joker)?,
    )?;
    exports.set("register_items", lua.create_function(register_items)?)?;

    Ok(exports)
}
