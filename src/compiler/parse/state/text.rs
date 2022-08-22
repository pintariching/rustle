use crate::compiler::interfaces::{BaseNode, Text};
use crate::compiler::parse::index::Parser;
use crate::compiler::parse::utils::decode_character_references;

pub fn text(parser: Parser) {
    let start = parser.index;
    let mut data = String::new();

    while parser.index < parser.template.len() && !parser.match_str("<") && !parser.match_str("{") {
        data += &parser.template[parser.index + 1..parser.template.len()];
    }

    let node: Text = Text {
        base_node: BaseNode {
            start,
            end: parser.index,
            node_type: "Text".to_string(),
            children: None,
            prop_name: Vec::new(),
        },
        // raw: data
        data: decode_character_references(data),
    };

    //parser.current().children.push(node);
}
