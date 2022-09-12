use std::{borrow::Borrow, collections::HashMap};

use map_macro::map;

use crate::compiler::{
    interfaces::{BaseNode, TemplateNode, TmpNode},
    parse::{
        errors::{self, Error},
        index::{Parser, StateReturn},
        utils::closing_tag_omitted,
    },
    utils::WHITESPACE,
};

fn trim_whitespace(block: &mut TemplateNode, trim_before: bool, trim_after: bool) {
    let children = block.get_children();

    if children.is_empty() {
        return;
    }

    if let Some(child) = children.first_mut() {
        match child {
            TemplateNode::Text(t) if trim_before => {
                t.data = t.data.trim_start().to_string();
                if t.data.is_empty() {
                    children.remove(0);
                }
            }
            _ => (),
        }
    }

    if let Some(child) = children.last_mut() {
        match child {
            TemplateNode::Text(t) if trim_after => {
                t.data = t.data.trim_end().to_string();
                if t.data.is_empty() {
                    children.pop();
                }
            }
            _ => (),
        }
    }

    // Only a mustache tag can contain an else and elseif statement?
    // {#if}
    // {#elseif}
    // {#else}
    match block {
        TemplateNode::MustacheTag(m) => {
            if let Some(b) = m.get_base_node().prop_name.get_mut("else") {
                trim_whitespace(b, trim_before, trim_after);
            }

            if let Some(b) = m.get_base_node().prop_name.get_mut("elseif") {
                trim_whitespace(b, trim_before, trim_after);
            }
        }
        _ => (),
    }
}

pub fn mustache(parser: &mut Parser) -> StateReturn {
    let start = parser.index;
    parser.index += 1;

    parser.allow_whitespace();

    // {/if}, {/each}, {/await} or {/key}
    if parser.eat("/", false, None) {
        let mut block = parser.current().clone();
        let mut expected: &str = "";

        if closing_tag_omitted(&block.get_name().unwrap(), None) {
            block.get_base_node().end = Some(start);
            parser.stack.pop();
            block = parser.current().clone();
        }

        if block.get_type() == "ElseBlock"
            || block.get_type() == "PendingBlock"
            || block.get_type() == "ThenBlock"
            || block.get_type() == "CatchBlock"
        {
            block.get_base_node().end = Some(start);
            parser.stack.pop();
            block = parser.current().clone();

            expected = "await";
        }

        match block.get_type().as_str() {
            "IfBlock" => expected = "if",
            "EachBlock" => expected = "each",
            "AwaitBlock" => expected = "await",
            "KeyBlock" => expected = "key",
            _ => {
                let error = Error::unexpected_block_close();
                parser.error(&error.code, &error.message, None);
            }
        }

        parser.eat(expected, true, None);
        parser.allow_whitespace();
        parser.eat("}", true, None);

        while let Some(_) = block.get_prop("elseif") {
            block.get_base_node().end = Some(parser.index);
            parser.stack.pop();
            block = parser.current().clone();

            if let Some(mut b) = block.get_prop("else") {
                b.get_base_node().end = Some(start);
            }
        }

        // strip leading/trailing whitespace as necessary
        let char_before = parser
            .template
            .chars()
            .nth(block.get_base_node().start.unwrap() - 1)
            .unwrap()
            .to_string();

        let char_after = parser
            .template
            .chars()
            .nth(parser.index)
            .unwrap()
            .to_string();

        let trim_before = !char_before.is_empty() || WHITESPACE.is_match(&char_before);
        let trim_after = !char_after.is_empty() || WHITESPACE.is_match(&char_after);

        trim_whitespace(&mut block, trim_before, trim_after);

        block.get_base_node().end = Some(parser.index);
        parser.stack.pop();
    } else if parser.eat(":else", false, None) {
        if parser.eat("if", false, None) {
            let error = Error::invalid_elseif();
            parser.error(&error.code, &error.message, None)
        }

        parser.allow_whitespace();

        // :else if
        if parser.eat("if", false, None) {
            let mut block = parser.current().clone();
            if block.get_type() != "IfBlock" {
                if parser
                    .stack
                    .iter()
                    .filter(|block| block.get_type() == "IfBlock")
                    .count()
                    > 0
                {
                    let error = Error::invalid_elseif_placement_unclosed_block(&block.to_string());
                    parser.error(&error.code, &error.message, None);
                } else {
                    let error = Error::invalid_elseif_placement_outside_if();
                    parser.error(&error.code, &error.message, None);
                }
            }

            parser.require_whitespace();

            // TODO
            // requires src::compiler::parse:read::expression to be written
            // let expression = read_expression(parser);

            parser.allow_whitespace();
            parser.eat("}", true, None);

            block.get_base_node().prop_name.insert(
                "else".to_string(),
                TemplateNode::BaseNode(BaseNode {
                    start: Some(parser.index),
                    end: None,
                    node_type: "ElseBlock".to_string(),
                    children: vec![TemplateNode::BaseNode(BaseNode {
                        start: Some(parser.index),
                        end: None,
                        node_type: "IfBlock".to_string(),
                        // TODO: the expression above is passed in here
                        expression: None,
                        children: Vec::new(),
                        elseif: true,
                        _else: false,
                        prop_name: HashMap::new(),
                    })],
                    prop_name: HashMap::new(),
                    expression: None,
                    elseif: false,
                    _else: false,
                }),
            );

            parser.stack.push(
                block
                    .get_base_node()
                    .prop_name
                    .get("else")
                    .unwrap()
                    .clone()
                    .get_children()
                    .first()
                    .unwrap()
                    .clone(),
            )
        }
        todo!()
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use map_macro::map;

    use crate::compiler::interfaces::{BaseNode, MustacheTag, Text};
    use crate::compiler::node::Node;

    use super::*;

    #[test]
    fn test_trim_whitespace_trim_before() {
        let mut sample = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![TemplateNode::Text(Text::new("   Hello ".to_string()))],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });

        trim_whitespace(&mut sample, true, false);
        let result = sample.get_children().first().unwrap().get_data();
        assert_eq!(result, "Hello ")
    }

    #[test]
    fn test_trim_whitespace_trim_after() {
        let mut sample = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![TemplateNode::Text(Text::new("   Hello   ".to_string()))],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });

        trim_whitespace(&mut sample, false, true);
        let result = sample.get_children().first().unwrap().get_data();
        assert_eq!(result, "   Hello")
    }

    #[test]
    fn test_trim_whitespace_trim_both() {
        let mut sample = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![TemplateNode::Text(Text::new("    Hello    ".to_string()))],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });

        trim_whitespace(&mut sample, true, true);
        let result = sample.get_children().first().unwrap().get_data();
        assert_eq!(result, "Hello")
    }

    #[test]
    fn test_trim_whitespace_shift_first_child() {
        let mut sample = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![
                    TemplateNode::Text(Text::new("    ".to_string())),
                    TemplateNode::Text(Text::new("Test".to_string())),
                ],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });

        trim_whitespace(&mut sample, true, true);
        let result = sample.get_children().len();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_trim_whitespace_pop_last_child() {
        let mut sample = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![
                    TemplateNode::Text(Text::new("Test".to_string())),
                    TemplateNode::Text(Text::new("  ".to_string())),
                ],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });

        trim_whitespace(&mut sample, true, true);
        let result = sample.get_children().len();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_trim_whitespace_else_node_shift_child() {
        let else_node = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![
                    TemplateNode::Text(Text::new("  ".to_string())),
                    TemplateNode::Text(Text::new("222".to_string())),
                ],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });
        let base_node = BaseNode {
            start: Some(0),
            end: Some(0),
            node_type: "MustacheTag".to_string(),
            children: vec![
                TemplateNode::Text(Text::new("  ".to_string())),
                TemplateNode::Text(Text::new("111".to_string())),
            ],
            expression: None,
            prop_name: map! {
                "else".to_string() => else_node
            },
            elseif: false,
            _else: false,
        };
        let mut sample =
            TemplateNode::MustacheTag(MustacheTag::new_with_base_node(base_node, Node::Empty));
        trim_whitespace(&mut sample, true, true);
        let result = sample.get_children().len();
        assert_eq!(result, 1);
        let result = sample.get_prop("else").unwrap().get_children().len();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_trim_whitespace_else_node_pop_child() {
        let else_node = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![
                    TemplateNode::Text(Text::new("222".to_string())),
                    TemplateNode::Text(Text::new("  ".to_string())),
                ],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });
        let base_node = BaseNode {
            start: Some(0),
            end: Some(0),
            node_type: "MustacheTag".to_string(),
            children: vec![
                TemplateNode::Text(Text::new("111".to_string())),
                TemplateNode::Text(Text::new("  ".to_string())),
            ],
            expression: None,
            prop_name: map! {
                "else".to_string() => else_node
            },
            elseif: false,
            _else: false,
        };
        let mut sample =
            TemplateNode::MustacheTag(MustacheTag::new_with_base_node(base_node, Node::Empty));
        trim_whitespace(&mut sample, true, true);
        let result = sample.get_children().len();
        assert_eq!(result, 1);
        let result = sample.get_prop("else").unwrap().get_children().len();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_trim_whitespace_elseif_node_shift_child() {
        let else_node = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![
                    TemplateNode::Text(Text::new("   ".to_string())),
                    TemplateNode::Text(Text::new("222".to_string())),
                ],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });
        let base_node = BaseNode {
            start: Some(0),
            end: Some(0),
            node_type: "MustacheTag".to_string(),
            children: vec![
                TemplateNode::Text(Text::new("   ".to_string())),
                TemplateNode::Text(Text::new("111".to_string())),
            ],
            expression: None,
            prop_name: map! {
                "elseif".to_string() => else_node
            },
            elseif: false,
            _else: false,
        };
        let mut sample =
            TemplateNode::MustacheTag(MustacheTag::new_with_base_node(base_node, Node::Empty));
        trim_whitespace(&mut sample, true, true);
        let result = sample.get_children().len();
        assert_eq!(result, 1);
        let result = sample.get_prop("elseif").unwrap().get_children().len();
        assert_eq!(result, 1)
    }

    #[test]
    fn test_trim_whitespace_elseif_node_pop_child() {
        let else_node = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: Some(0),
                end: Some(0),
                node_type: "Text".to_string(),
                children: vec![
                    TemplateNode::Text(Text::new("222".to_string())),
                    TemplateNode::Text(Text::new("   ".to_string())),
                ],
                prop_name: Default::default(),
                expression: None,
                elseif: false,
                _else: false,
            },
            data: " Hello ".to_string(),
        });
        let base_node = BaseNode {
            start: Some(0),
            end: Some(0),
            node_type: "MustacheTag".to_string(),
            children: vec![
                TemplateNode::Text(Text::new("111".to_string())),
                TemplateNode::Text(Text::new("   ".to_string())),
            ],
            expression: None,
            prop_name: map! {
                "elseif".to_string() => else_node
            },
            elseif: false,
            _else: false,
        };
        let mut sample =
            TemplateNode::MustacheTag(MustacheTag::new_with_base_node(base_node, Node::Empty));
        trim_whitespace(&mut sample, true, true);
        let result = sample.get_children().len();
        assert_eq!(result, 1);
        let result = sample.get_prop("elseif").unwrap().get_children().len();
        assert_eq!(result, 1)
    }
}
