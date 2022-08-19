use crate::compiler::parse::index::Parser;

pub fn fragment(parser: Parser) {
    if parser.match_str("<") {
        //return tag;
    }

    if parser.match_str("{") {
        //return mustache;
    }

    //text
}
