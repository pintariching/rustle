pub fn full_char_at(str: &str, i: usize) -> char {
    str.chars().nth(i).unwrap()
}

#[cfg(test)]
mod tests {
    use super::full_char_at;

    #[test]
    fn test_full_char_at() {
        let string = "a long string!";

        // letter g
        assert_eq!(full_char_at(string, 5), 'g');

        // exclamation mark !
        assert_eq!(full_char_at(string, 13), '!');

        // space
        assert_eq!(full_char_at(string, 1), ' ');
    }
}
