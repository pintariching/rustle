//Named full_char_code_at in javascript compiler
pub fn full_char_code_at(str: &str, i: usize) -> u32 {
    str.encode_utf16().nth(i).unwrap() as u32
}
