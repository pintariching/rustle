//Named full_char_code_at in javascript compiler
pub fn word(str: &str, i: usize) -> u8 {
    let code = &str[i..i+1].as_bytes()[0];
    
    if *code <= 0xd7ff || *code >= 0xe000 {
        return *code
    }

    let next = &str[i + 1..i+2].as_bytes()[0];
    (code << 10) + next - 0x35fdc00
}