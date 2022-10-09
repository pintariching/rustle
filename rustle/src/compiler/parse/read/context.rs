use std::fmt::format;

use crate::compiler::parse::index::Parser;
use crate::compiler::parse::utils::{is_bracket_close, is_bracket_open, is_bracket_pair, get_bracket_close};
use crate::compiler::utils::{full_char_at, full_char_code_at};
use crate::compiler::parse::errors::Error;
use crate::compiler::parse::acorn::parse_expression_at;

use regex::Regex;
use swc_ecma_ast::{Ident, Expr};
use swc_estree_ast::{PatternLike, Identifier, BaseNode};

pub fn read_context(mut parser: Parser) -> (PatternLike, u32, u32) {
    let start = parser.index;
    let mut i = parser.index as usize;

    let code = full_char_at(&parser.template, i);

    //Why is code a char above?
    let otherCode = full_char_code_at(&parser.template, i);

    if Ident::is_valid_start(code) {
        let base = BaseNode { leading_comments: Vec::new(), inner_comments: Vec::new(), trailing_comments: Vec::new(), start: Some(start as u32), end: Some(parser.index as u32), range: None, loc: None };
        let id = Identifier {
            base,
            name: parser.read_identifier(Some(false)).unwrap().into(),
            decorators: None,
            optional: None,
            type_annotation: None

        };
        return (PatternLike::Id(id), start as u32, parser.index as u32)
    }

    // use is_bracked_close?
    if !is_bracket_open(code) {
        let error = Error::unexpected_token_destructure();
        parser.error(&error.code, &error.message, None);
    }

    let mut bracket_stack: Vec<char> = Vec::new();
    bracket_stack.push(code);
    // Looks like this in javascript:
    // i += code <= 0xffff ? 1 : 2;
    if otherCode <= 0xffff {
        i += 1;
    } else {
        i += 2;
    }

    while i < parser.template.len() {
        let code = full_char_at(&parser.template, i);
        if is_bracket_open(code) {
            bracket_stack.push(code);
        } else if is_bracket_close(code) {
            if !is_bracket_pair(bracket_stack[bracket_stack.len() - 1], code) {
                let close_bracket = get_bracket_close(bracket_stack[bracket_stack.len() - 1]) as u16;
                let error = Error::unexpected_token(
                    &String::from_utf16(&[close_bracket]).unwrap()
                );
                parser.error(&error.code, &error.message, None);
            }
            bracket_stack.pop();
            if bracket_stack.is_empty() {
                // javascript black magic?
                // i += code <= 0xffff ? 1 : 2;
                if otherCode <= 0xffff {
                    i += 1;
                } else {
                    i += 2;
                }
                break;
            }
        }

        if otherCode <= 0xffff {
            i += 1;
        } else {
            i += 2;
        }
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

    // TODO - Waiting for rustle-code-red to me implemented
    // return (parse_expression_at(
    // 		`${space_with_newline}(${pattern_string} = 1)`,
    // 		start - 1
    // 	) as any).left;

    let val = *parse_expression_at(format!("{}({} = 1)", space_with_newline, pattern_string), start - 1);

    if let Expr::Ident(s) = val {
        todo!()
    }

}
