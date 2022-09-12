use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref WHITESPACE: Regex = Regex::new("[ \t\r\n]").unwrap();
    pub static ref START_WHITESPACE: Regex = Regex::new("^[ \t\r\n]*").unwrap();
    pub static ref END_WHITESPACE: Regex = Regex::new("[ \t\r\n]*$").unwrap();
    pub static ref START_NEWLINE: Regex = Regex::new("^\r?\n").unwrap();
    pub static ref DIMENSIONS: Regex = Regex::new("^(?:offset|client)(?:Width|Height)$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::{DIMENSIONS, END_WHITESPACE, START_NEWLINE, START_WHITESPACE, WHITESPACE};

    #[test]
    fn test_whitespace_regex() {
        let samples = vec![" ", "\t", "\r", "\n", "	"];

        for s in samples {
            assert!(WHITESPACE.is_match(s));
        }
    }

    #[test]
    fn test_start_whitespace_regex() {
        let samples = vec![" word", "\tword", "\rword", "\nword", "	word"];

        for s in samples {
            assert!(START_WHITESPACE.is_match(s));
        }
    }

    #[test]
    fn test_end_whitespace_regex() {
        let samples = vec!["word ", "word\t", "word\r", "word\n", "	word"];

        for s in samples {
            assert!(END_WHITESPACE.is_match(s));
        }
    }

    #[test]
    fn test_start_newline_regex() {
        let samples = vec!["\rword", "\nword"];

        for s in samples {
            assert!(END_WHITESPACE.is_match(s));
        }
    }

    #[test]
    fn test_dimensions_regex() {
        let samples = vec!["offsetWidth", "offsetHeight", "clientWidth", "clientHeight"];

        for s in samples {
            assert!(DIMENSIONS.is_match(s));
        }
    }
}
