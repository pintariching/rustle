use crate::compiler::interfaces::{BaseNode, Text, TemplateNode};
use crate::compiler::parse::index::{Parser, StateReturn};
use crate::compiler::parse::utils::decode_character_references;

pub fn text(parser: &mut Parser) -> StateReturn {
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

    let tempNode = parser.current().unwrap();
    let base_node = tempNode.get_base_node();

    base_node.children.as_mut().unwrap().push(TemplateNode::Text((node)));

    StateReturn::None
}
