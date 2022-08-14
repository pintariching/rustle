use crate::compiler::parse::index::Parser;

use super::{mustache, tag, text};

pub fn fragment(parser: Parser) {
    if parser.match_str("<") {
        //return tag;
    }

    if parser.match_str("{") {
        //return mustache;
    }

    //text
}
