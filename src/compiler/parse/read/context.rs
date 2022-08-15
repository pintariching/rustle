use crate::compiler::parse::index::Parser;
use crate::compiler::parse::utils::{is_bracket_close, is_bracket_open, is_bracket_pair};
use crate::compiler::utils::full_char_at;

use regex::Regex;
use swc_ecma_ast::Ident;
use swc_estree_ast::Pattern;

pub fn read_context(mut parser: Parser) -> (Pattern, i32, i32) {
    let start = parser.index;
    let i = parser.index as usize;

    let code = full_char_at(&parser.template, i);

    if Ident::is_valid_start(code) {
        // TODO
    }

    // use is_bracked_close?
    if !is_bracket_open(code) {
        // TODO
    }

    let mut bracket_stack: Vec<char> = Vec::new();
    bracket_stack.push(code);
    // javascript black magic?
    // i += code <= 0xffff ? 1 : 2;

    while i < parser.template.len() {
        let code = full_char_at(&parser.template, i);
        if is_bracket_open(code) {
            bracket_stack.push(code);
        } else if is_bracket_close(code) {
            if !is_bracket_pair(bracket_stack[bracket_stack.len() - 1], code) {
                // TODO: throw error
                // parser.error(
                // parser_errors.unexpected_token(
                // 	String.fromCharCode(get_bracket_close(bracket_stack[bracket_stack.length - 1]))
                // )
            }
            bracket_stack.pop();
            if bracket_stack.is_empty() {
                // javascript black magic?
                // i += code <= 0xffff ? 1 : 2;
                // break;
            }
        }

        //i += code <= 0xffff ? 1 : 2;
    }

    parser.index = i;

    let pattern_string = &parser.template[start as usize..i];

    // the length of the `space_with_newline` has to be start - 1
    // because we added a `(` in front of the pattern_string,
    // which shifted the entire string to right by 1
    // so we offset it by removing 1 character in the `space_with_newline`
    // to achieve that, we remove the 1st space encountered,
    // so it will not affect the `column` of the node
    let mut space_with_newline = Regex::new("/[^\n]/g")
        .unwrap()
        .replace(&parser.template[0..start], " ")
        .to_string();

    let first_space = space_with_newline.find(" ").unwrap();
    space_with_newline = format!(
        "{}{}",
        &space_with_newline[0..first_space],
        &space_with_newline[first_space + 1..space_with_newline.len()]
    );

    // TODO
    // return (parse_expression_at(
    // 		`${space_with_newline}(${pattern_string} = 1)`,
    // 		start - 1
    // 	) as any).left;
    todo!()
}
