use crate::compiler::parse::index::Parser;
use crate::compiler::parse::utils::is_bracket_open;
use crate::compiler::utils::full_char_at;

use swc_ecma_ast::Ident;
use swc_estree_ast::Pattern;

pub fn read_context(parser: Parser) -> (Pattern, i32, i32) {
    let start = parser.index;
    let i = parser.index;

    let code = full_char_at(&parser.template, i as usize);

    if Ident::is_valid_start(code) {
        // TODO
    }

    if !is_bracket_open(code) {
        // TODO
    }

    let bracket_stack = [code];

    // javascript black magic?
    // i += code <= 0xffff ? 1 : 2;

    todo!()
}
