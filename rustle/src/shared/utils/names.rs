use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VOID_ELEMENT_NAMES: Regex = Regex::new(
        "/^(?:area|base|br|col|command|embed|hr|img|input|keygen|link|meta|param|source|track|wbr)$/"
    )
    .unwrap();
}

pub fn is_void(name: &str) -> bool {
    VOID_ELEMENT_NAMES.is_match(name) || name.to_lowercase() == "!doctype"
}
