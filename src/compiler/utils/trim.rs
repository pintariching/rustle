use super::patterns::{END_WHITESPACE, START_WHITESPACE};

pub fn trim_start(str: &str) -> &str {
    &START_WHITESPACE.replace(str, "")
}

pub fn trim_end(str: &str) -> &str {
    &END_WHITESPACE.replace(str, "")
}
