use crate::compiler::interfaces::{BaseNode, TemplateNode, Text};
use crate::compiler::parse::index::{Parser, StateReturn};
use crate::compiler::parse::utils::decode_character_references;
use std::collections::HashMap;

pub fn text(parser: &mut Parser) -> StateReturn {
    let start = parser.index;
    let mut data = String::new();

    while parser.index < parser.template.len() && !parser.match_str("<") && !parser.match_str("{") {
        data += &parser.template[parser.index + 1..parser.template.len()];
    }

    let node: Text = Text {
        base_node: BaseNode {
            start: Some(start),
            end: Some(parser.index),
            node_type: "Text".to_string(),
            children: Vec::new(),
            prop_name: HashMap::new(),
            expression: None,
            elseif: false,
            _else: false,
        },
        // raw: data
        data: decode_character_references(data),
    };

    parser
        .current()
        .get_children()
        .push(TemplateNode::Text(node));

    StateReturn::None
}
