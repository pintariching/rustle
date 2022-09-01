use crate::compiler::parse::index::Parser;
use crate::compiler::parse::index::StateReturn;
use crate::compiler::parse::state::{mustache, tag, text::text};

pub fn fragment(parser: &mut Parser) -> StateReturn {
    if parser.match_str("<") {
        //TODO implement tag.rs
        // return StateReturn::Ok(tag)
    }

    if parser.match_str("{") {
        //TODO implement mustache.rs
        // return StateReturn::Ok(mustache)
    }

    return text(parser);
}
