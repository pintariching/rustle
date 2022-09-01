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

pub fn get_bracket_close(open: char) -> char {
    if open == SQUARE_BRACKET_OPEN {
        return SQUARE_BRACKET_CLOSE;
    }

    if open == CURLY_BRACKET_OPEN {
        return CURLY_BRACKET_CLOSE;
    }

    unreachable!()
}
