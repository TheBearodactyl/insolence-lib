use mlua::prelude::*;

pub(crate) fn between(_: &Lua, (num, min, max): (f64, f64, f64)) -> LuaResult<bool> {
    Ok(num >= min && num <= max)
}

pub(crate) fn within(_: &Lua, (x, y): (f64, f64)) -> LuaResult<bool> {
    Ok((x - y).abs() <= x)
}

pub(crate) fn wave_number(_: &Lua, num: f64) -> LuaResult<f64> {
    if num == 0.0 {
        Ok(1.0)
    } else if num > 0.0 {
        Ok(-num - 1.0)
    } else {
        Ok(-num + 1.0)
    }
}

pub(crate) fn clamp(_: &Lua, (num, min, max): (f64, f64, f64)) -> LuaResult<f64> {
    if num < min {
        Ok(min)
    } else if num > max {
        Ok(max)
    } else {
        Ok(num)
    }
}

pub(crate) fn exponentiate(lua: &Lua, (mut base, mut power): (f64, f64)) -> LuaResult<f64> {
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
