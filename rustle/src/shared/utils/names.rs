use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VOID_ELEMENT_NAMES: Regex = Regex::new(
        "^(?:area|base|br|col|command|embed|hr|img|input|keygen|link|meta|param|source|track|wbr)$"
    )
    .unwrap();
}

pub fn is_void(name: &str) -> bool {
    VOID_ELEMENT_NAMES.is_match(name) || name.to_lowercase() == "!doctype"
}

#[cfg(test)]
mod tests {
    use super::{is_void, VOID_ELEMENT_NAMES};

    #[test]
    fn test_void_element_names_regex() {
        let samples = vec![
            "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen",
            "link", "meta", "param", "source", "track", "wbr",
        ];

        for s in samples {
            assert!(VOID_ELEMENT_NAMES.is_match(s));
        }
    }

    #[test]
    fn test_is_void_pass() {
        let samples = vec![
            "area", "base", "br", "col", "command", "embed", "hr", "img", "input", "keygen",
            "link", "meta", "param", "source", "track", "wbr", "!doctype",
        ];

        for s in samples {
            assert!(is_void(s));
        }
    }

    #[test]
    fn test_is_void_fail() {
        let samples = vec!["test", "doctype", "bs", "break"];

        for s in samples {
            assert!(!is_void(s));
        }
    }
}
