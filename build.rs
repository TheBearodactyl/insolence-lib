use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let luals_docs = "--- @class InsolenceLib # A utility class for Insolence
--- @field between fun(num: number, min: number, max: number): boolean # Gets whether a number is between min and max
--- @field largest_val fun(tbl: table): any # Returns the largest number value in a given table
--- @field average_table_amt fun(tbl: table): number # Gets the average of all number values in a given table
--- @field reverse_table fun(tbl: table): table # Reverses the order of key/value pairs in a table
--- @field capitalize fun(word: string): string # Capitalizes the first letter of a word
--- @field every_day_im_shufflin fun(tbl: table): table # Shuffles the keys and values of the given table
--- @field rand_mem_addr fun(): string # Returns a random (and most probably fake) memory address based on the host devices pointer width
--- @field placeholder_sprite fun(): { x: number, y: number } # Returns the X and Y positions of the placeholder sprite
--- @field random_str fun(len: number, char_set?: string): string # Returns a random string of the specified length and using only the chars in `char_set` (optional)
--- @field rand_hex_code fun(): string # Returns a random color code
--- @field rand_int fun(min: integer, max: integer): integer # Returns a random integer
--- @field rand_num fun(min: number, max: number): integer # Returns a random number
--- @field random_table_of_strs fun(str_len: integer, tbl_len: integer): table<string> # Returns a table of randomly generated strings
--- @field mod_vals fun(lua: &Lua, input: number | table, modifier: number): table | number
--- @field wave_number fun(num: number): number
--- @field clamp fun(num: number, min: number, max: number): number # Clamps a given number between min and max
--- @field chance fun(percent_chance: number): boolean
--- @field exponentiate fun(base: number, power: number): number
--- @field boobs_sprite fun(mod_cfg: Mod): { x: number, y: number }
local insolence = require(\"insolence\");

Copy the above to the file you're using the lib from then run the following command on it:
sed -i 's/warning: insolencelib@0\\.1\\.0: //g' <your lua file>
";

    for line in luals_docs.lines() {
        println!("cargo::warning={}", line);
    }
}

