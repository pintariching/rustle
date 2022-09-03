use crate::compiler::{
    interfaces::{TemplateNode, TmpNode},
    parse::index::{Parser, StateReturn},
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
            if let Some(b) = m.get_base_node().props.get_mut("else") {
                trim_whitespace(b, trim_before, trim_after);
            }

            if let Some(b) = m.get_base_node().props.get_mut("elseif") {
                trim_whitespace(b, trim_before, trim_after);
            }
        }
        _ => (),
    }
}

pub fn mustache(parser: &mut Parser) -> StateReturn {
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
            prop_name: Default::default(),
            else_if: false,
            expression: None,
            props: map! {
                "else".to_string() => else_node
            },
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
            prop_name: Default::default(),
            else_if: false,
            expression: None,
            props: map! {
                "else".to_string() => else_node
            },
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
            prop_name: Default::default(),
            else_if: false,
            expression: None,
            props: map! {
                "elseif".to_string() => else_node
            },
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
                else_if: false,
                expression: None,
                props: HashMap::new(),
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
            prop_name: Default::default(),
            else_if: false,
            expression: None,
            props: map! {
                "elseif".to_string() => else_node
            },
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
