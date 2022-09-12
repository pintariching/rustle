//Named full_char_code_at in javascript compiler
pub fn full_char_code_at(str: &str, i: usize) -> u32 {
    str.encode_utf16().nth(i).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::full_char_code_at;

    #[test]
    fn test_full_char_code_at() {
        let string = "a long string!";

        // utf16 table at https://asecuritysite.com/coding/asc2
        // letter g
        assert_eq!(full_char_code_at(string, 5), 103);

        // exclamation mark !
        assert_eq!(full_char_code_at(string, 13), 33);

        // space
        assert_eq!(full_char_code_at(string, 1), 32);
    }
}
