use std::collections::HashMap;

use crate::compiler::{
    interfaces::{BaseNode, Style, StyleContent},
    node::Node,
    parse::{errors::Error, index::Parser},
};
use lazy_static::lazy_static;
use regex::Regex;
use swc_common::BytePos;
use swc_css::{
    ast::Stylesheet,
    parser::{parse_str, parser::ParserConfig},
};

lazy_static! {
    static ref STYLE_REGEX: Regex = Regex::new("</style\\s*>").unwrap();
}

pub fn read_style(parser: &mut Parser, start: usize, attributes: Vec<Node>) -> Style {
    let content_start = parser.index;
    let styles = parser.read_until(STYLE_REGEX.clone(), Some(Error::unclosed_style()));

    if parser.index >= parser.template.len() {
        let error = Error::unclosed_style();
        parser.error(&error.code, &error.message, None)
    }

    let content_end = parser.index;

    let ast: Stylesheet = parse_str(
        styles.as_str(),
        BytePos(content_start as u32),
        BytePos(content_end as u32),
        ParserConfig::default(),
        &mut Vec::new(),
    )
    .unwrap();

    // TODO: Tidy up AST

    let _ = parser.read(STYLE_REGEX.clone());
    let end = parser.index;

    return Style {
        base_node: BaseNode {
            node_type: "Style".to_string(),
            start: Some(start),
            end: Some(end),
            children: Vec::new(),
            prop_name: HashMap::new(),
            _else: false,
            elseif: false,
            expression: None,
        },
        content: StyleContent {
            start: content_start,
            end: content_end,
            styles: styles,
        },
        attributes,
        children: ast.rules,
    };
}

#[cfg(test)]
mod tests {
    use super::STYLE_REGEX;

    #[test]
    fn test_style_regex() {
        let samples = vec!["</style>", "</style >"];

        for s in samples {
            assert!(STYLE_REGEX.is_match(s));
        }
    }
}
