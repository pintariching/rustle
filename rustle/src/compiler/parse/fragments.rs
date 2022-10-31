use lazy_static::lazy_static;
use regex::Regex;
use swc_css_ast::Stylesheet;
use swc_ecma_ast::{Expr, Script};

use super::parser::Parser;
use super::swc_helpers::{parse_expression_at, swc_parse_css, swc_parse_javascript};
use crate::compiler::{AttributeValue, Fragment, RustleAttribute, RustleElement, RustleText};

lazy_static! {
    // for HTML elements -> <h1> matches "h1"
    static ref ELEMENT_TAG_NAME: Regex = Regex::new("[a-z1-9]").unwrap();

    // for matching the start of an attribute or the end of a tag
    static ref ATTRIBUTE_NAME: Regex = Regex::new("[^=>]").unwrap();

    // for matching text inside tags <h1>some text</h1> -> "some text"
    static ref READ_TEXT: Regex = Regex::new("[^<{]").unwrap();

    // for reading attribute values -> class="p-5" -> "p-5"
    static ref ATTRIBUTE_VALUE: Regex = Regex::new("[a-z0-9-\\s]").unwrap();
}

/// Parses fragments given an end condition as a closure.
///
/// # Arguments
/// * `parser` - The `parser` struct containing the content to parse
/// * `condition` - A closure that accepts a `Parser` argument and checks when to end the parsing
pub fn parse_fragments<F: Fn(&mut Parser) -> bool>(
    parser: &mut Parser,
    condition: F,
) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    while condition(parser) {
        if let Some(fragment) = parse_fragment(parser) {
            fragments.push(fragment);
        }
    }

    fragments
}

/// Parses a fragment given a parser struct. If it can't parse
/// it returns `None`.
///
/// # Arguments
/// * `parser` - The `parser` struct containing the content to parse.
pub fn parse_fragment(parser: &mut Parser) -> Option<Fragment> {
    if let Some(script) = parse_script(parser) {
        return Some(Fragment::Script(script));
    }

    if let Some(style) = parse_style(parser) {
        return Some(Fragment::Style(style));
    }

    if let Some(element) = parse_element(parser) {
        return Some(Fragment::Element(element));
    }

    if let Some(expression) = parse_expression(parser) {
        return Some(Fragment::Expression(expression));
    }

    if let Some(text) = parse_text(parser) {
        return Some(Fragment::Text(text));
    }

    None
}

/// Checks if the index starts at a `<script>` tag and parses
/// the content between it and a `</script>` tag using SWC
/// and returns a `swc_ecma_ast::Script`.
///
/// Sets the `parser.index` to the end of the closing `</script>` tag.
///
/// Returns `None` if the current index doesn't start at a `<script>` tag.
fn parse_script(parser: &mut Parser) -> Option<Script> {
    if parser.match_str("<script>") {
        parser.eat("<script>");
        let start_index = parser.index;
        let end_index = parser.content.find("</script>").unwrap();
        let code = parser.content.get(start_index..end_index).unwrap();
        let script = swc_parse_javascript(code);

        parser.index = end_index;
        parser.eat("</script>");

        return Some(script);
    }

    None
}

/// Checks if the index starts at a `<style>` tag and returns
/// the content between it and a `</style>` tag as a `swc_css_ast::Stylesheet`.
///
/// Sets the `parser.index` to the end of the closing `</style>` tag.
///
/// Returns `None` if the current index doesn't start at a `<style>` tag.
fn parse_style(parser: &mut Parser) -> Option<Stylesheet> {
    if parser.match_str("<style>") {
        parser.eat("<style>");
        let start_index = parser.index;
        let end_index = parser.content.find("</style>").unwrap();
        let style = parser.content.get(start_index..end_index).unwrap();

        let stylesheet = swc_parse_css(style);

        parser.index = end_index;
        parser.eat("</style>");

        return Some(stylesheet);
    }

    None
}

/// Checks if the index starts at an opening `<` tag
/// and parses the tag name and attributes.
///
/// A valid element tag looks like `<button on:click={action}>Button</button>`
/// or a simple `<div></div>.
///
/// Also parses recursively into elements like `<div><ul><li></li></ul></div>.
///
/// Sets the `parser.index` to the ending of the element.
fn parse_element(parser: &mut Parser) -> Option<RustleElement> {
    if parser.match_str("<") {
        parser.eat("<");

        let tag_name = parser.read_while_matching(&ELEMENT_TAG_NAME);
        let attributes = parse_attribute_list(parser);
        parser.eat(">");

        let end_tag = format!("</{}>", tag_name);

        let element = Some(RustleElement {
            name: tag_name,
            attributes: attributes,
            fragments: parse_fragments(parser, |parser| !parser.match_str(&end_tag)),
        });

        parser.eat(end_tag.as_str());
        return element;
    }

    None
}

/// Checks if the index is at a curly brace `{` and parses the expression
/// at the index untill the next closing curly brace `}`.
///
/// Sets the `parser.index` to the closing curly brace `}` index.
fn parse_expression(parser: &mut Parser) -> Option<Expr> {
    if parser.match_str("{") {
        parser.eat("{");
        let expr = parse_expression_at(parser);
        parser.eat("}");

        return Some(expr);
    }

    None
}

/// Parses text between tags for example `<div>some text</div>`
fn parse_text(parser: &mut Parser) -> Option<RustleText> {
    let text = parser.read_while_matching(&READ_TEXT);

    if text.trim() != "" {
        return Some(RustleText { data: text.into() });
    }
    None
}

/// Parses all the attributes inside a tag untill the closing `>`
/// for example `on:click={action}`
fn parse_attribute_list(parser: &mut Parser) -> Vec<RustleAttribute> {
    let mut attributes = Vec::new();
    parser.skip_whitespace();

    while !parser.match_str(">") {
        attributes.push(parse_attribute(parser));
        parser.skip_whitespace();
    }

    attributes
}

/// Gets the attribute name and the value between curly braces
/// `on:click={action}` -> `on:click`, `AttributeValue::Expr(action)`
/// `class="py-5"` -> `class`, `AttributeValue::String("py-5".to_string())`
fn parse_attribute(parser: &mut Parser) -> RustleAttribute {
    let name = parser.read_while_matching(&ATTRIBUTE_NAME);

    if parser.match_str("={") {
        parser.eat("={");
        let value = parse_expression_at(parser);
        parser.eat("}");

        return RustleAttribute {
            name,
            value: AttributeValue::Expr(value),
        };
    }

    if parser.match_str("=\"") || parser.match_str("='") {
        parser.eat("=");
        if parser.match_str("\"") {
            parser.eat("\"");
        } else if parser.match_str("'") {
            parser.eat("'");
        }

        let value = parser.read_while_matching(&ATTRIBUTE_VALUE);

        if parser.match_str("\"") {
            parser.eat("\"");
        } else if parser.match_str("'") {
            parser.eat("'");
        }

        return RustleAttribute {
            name,
            value: AttributeValue::String(value),
        };
    } else {
        RustleAttribute {
            name,
            value: AttributeValue::String(String::new()),
        }
    }
}
