#![allow(clippy::only_used_in_recursion)]

mod math;
mod rng;
mod tbl;
mod text;
mod utils;

use math::*;
use rng::*;
use tbl::*;
use text::*;
use utils::*;

mkmodule! {
    libinsolence,
    funcs [
        between,
        boobs_sprite,
        largest_val,
        average_table_amt,
        reverse_table,
        capitalize,
        every_day_im_shufflin,
        mod_vals,
        random_str,
        rand_hex_code,
        exponentiate,
        rand_int,
        rand_num,
        wave_number,
        clamp,
        chance,
        within,
        random_table_of_strs,
        rand_mem_addr,
        placeholder_sprite,
        include_content,
        include,
        word_to_color,
        random_joker,
        rand_table_of_hex_codes,
        is_rigged_cryptid,
        mod_cond,
        count_num_of_joker,
        register_items,
        register_items_adv,
        animate_center
    ]
}
