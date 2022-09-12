const SQUARE_BRACKET_OPEN: char = '[';
const SQUARE_BRACKET_CLOSE: char = ']';
const CURLY_BRACKET_OPEN: char = '{';
const CURLY_BRACKET_CLOSE: char = '}';

pub fn is_bracket_open(code: char) -> bool {
    code == SQUARE_BRACKET_OPEN || code == CURLY_BRACKET_OPEN
}

pub fn is_bracket_close(code: char) -> bool {
    code == SQUARE_BRACKET_CLOSE || code == CURLY_BRACKET_CLOSE
}

pub fn is_bracket_pair(open: char, close: char) -> bool {
    (open == SQUARE_BRACKET_OPEN && close == SQUARE_BRACKET_CLOSE)
        || (open == CURLY_BRACKET_OPEN && close == CURLY_BRACKET_CLOSE)
}

pub fn get_bracket_close(open: char) -> Option<char> {
    if open == SQUARE_BRACKET_OPEN {
        return Some(SQUARE_BRACKET_CLOSE);
    }

    if open == CURLY_BRACKET_OPEN {
        return Some(CURLY_BRACKET_CLOSE);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{get_bracket_close, is_bracket_close, is_bracket_open, is_bracket_pair};

    #[test]
    fn test_bracket_open() {
        assert!(is_bracket_open('['));
        assert!(is_bracket_open('{'));
        assert!(!is_bracket_open(']'));
        assert!(!is_bracket_open('}'));
        assert!(!is_bracket_open('a'));
        assert!(!is_bracket_open(' '));
    }

    #[test]
    fn test_is_bracket_close() {
        assert!(is_bracket_close(']'));
        assert!(is_bracket_close('}'));
        assert!(!is_bracket_close('['));
        assert!(!is_bracket_close('{'));
        assert!(!is_bracket_close('a'));
        assert!(!is_bracket_close(' '));
    }

    #[test]
    fn test_is_bracket_pair() {
        assert!(is_bracket_pair('[', ']'));
        assert!(is_bracket_pair('{', '}'));
        assert!(!is_bracket_pair(']', '['));
        assert!(!is_bracket_pair(']', ']'));
        assert!(!is_bracket_pair('}', '{'));
        assert!(!is_bracket_pair('a', 'b'));
        assert!(!is_bracket_pair('+', '2'));
    }

    #[test]
    fn test_get_bracket_close() {
        assert_eq!(get_bracket_close('['), Some(']'));
        assert_eq!(get_bracket_close('{'), Some('}'));
        assert_eq!(get_bracket_close(']'), None);
        assert_eq!(get_bracket_close('}'), None);
        assert_eq!(get_bracket_close('a'), None);
        assert_eq!(get_bracket_close(' '), None);
    }
}
