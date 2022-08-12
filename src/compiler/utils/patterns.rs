use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref WHITESPACE: Regex = Regex::new("[ \t\r\n]").unwrap();
    pub static ref START_WHITESPACE: Regex = Regex::new("^[ \t\r\n]*").unwrap();
    pub static ref END_WHITESPACE: Regex = Regex::new("[ \t\r\n]*$").unwrap();
    pub static ref START_NEWLINE: Regex = Regex::new("^\r?\n").unwrap();
    pub static ref DIMENSIONS: Regex = Regex::new("^(?:offset|client)(?:Width|Height)$").unwrap();
}
