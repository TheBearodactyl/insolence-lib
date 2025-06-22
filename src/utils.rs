use mlua::prelude::*;

pub(crate) fn placeholder_sprite(lua: &Lua, _: ()) -> LuaResult<LuaTable> {
    let spr_loc = lua.create_table()?;

    spr_loc.set("x", 19)?;
    spr_loc.set("y", 0)?;

    Ok(spr_loc)
}

pub(crate) fn boobs_sprite(lua: &Lua, mod_cfg: LuaValue) -> LuaResult<LuaTable> {
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

pub(crate) fn include_content(lua: &Lua, (name, ty): (String, String)) -> LuaResult<()> {
    let path = format!("src/content/{}/{}.lua", ty, name);

    let smods: LuaTable = lua.globals().get("SMODS")?;
    let load_file: LuaFunction = smods.get("load_file")?;
    let func: LuaFunction = load_file.call::<LuaFunction>(path)?;

    func.call::<LuaFunction>(())?;

    Ok(())
}

pub(crate) fn include(lua: &Lua, path: String) -> LuaResult<()> {
    let smods: LuaTable = lua.globals().get("SMODS")?;
    let load_file: LuaFunction = smods.get("load_file")?;
    let func: LuaFunction = load_file.call::<LuaFunction>(path)?;

    func.call::<LuaFunction>(())?;

    Ok(())
}

pub(crate) fn word_to_color(_: &Lua, word: String) -> LuaResult<String> {
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

pub(crate) fn is_rigged_cryptid(lua: &Lua, card: LuaTable) -> LuaResult<bool> {
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

pub(crate) fn mod_cond(
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

pub(crate) fn count_num_of_joker(lua: &Lua, (prefix, key): (String, String)) -> LuaResult<u32> {
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

pub(crate) fn register_items(lua: &Lua, (items, path): (Vec<String>, String)) -> LuaResult<()> {
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

pub(crate) fn animate_center(
    lua: &Lua,
    (center_id, center_dt_key, dt, interval, tx, ty, mx, my): (
        String,
        String,
        f64,
        f64,
        i32,
        i32,
        i32,
        i32,
    ),
) -> LuaResult<()> {
    let globals = lua.globals();
    let current_dt = match globals.get::<LuaValue>(center_dt_key.clone())? {
        LuaValue::Number(n) => n,
        _ => 0.0,
    };

    let new_dt = current_dt + dt;

    if new_dt > interval {
        globals.set(center_dt_key, 0.0)?;

        let g_table: LuaTable = globals.get("G")?;
        let p_centers: LuaTable = g_table.get("P_CENTERS")?;

        if let Ok(center_obj) = p_centers.get::<LuaTable>(center_id) {
            let pos: LuaTable = center_obj.get("pos")?;
            let x: i32 = pos.get("x")?;
            let y: i32 = pos.get("y")?;

            if x == tx && y == ty {
                pos.set("x", 0)?;
                pos.set("y", 0)?;
            } else if x < mx {
                pos.set("x", x + 1)?;
            } else if y < my {
                pos.set("x", 0)?;
                pos.set("y", y + 1)?;
            }

            center_obj.set("pos", pos)?;
        }
    } else {
        globals.set(center_dt_key, new_dt)?;
    }

    Ok(())
}

#[macro_export]
macro_rules! mkmodule {
    ($modname:ident, funcs [ $($func:ident),* $(,)? ]) => {
        #[mlua::lua_module]
        fn $modname(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
            let exports = lua.create_table()?;

            $(
                exports.set(stringify!($func), lua.create_function($func)?)?;
            )*

            Ok(exports)
        }
    };
}
