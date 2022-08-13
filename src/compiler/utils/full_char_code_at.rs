//Named full_char_code_at in javascript compiler
pub fn full_char_code_at(str: &str, i: usize) -> u32 {
    str.chars().nth(i).unwrap() as u32
}