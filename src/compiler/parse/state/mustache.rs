use crate::compiler::interfaces::{Children, TemplateNode, Text, TmpNode};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub fn trim_whitespace(
    block: &mut TemplateNode,
    trim_before: bool,
    trim_after: bool,
) -> Option<()> {
    let children = block.get_children();

    if children.is_empty() {
        return None;
    }

    let mut is_data_empty = false;
    if let Some(child) = children.first() {
        match child {
            TemplateNode::Text(t) if trim_before => {
                t.data = t.data.trim_start().to_string();
                if t.data.is_empty() {
                    block.shift_children();
                }
            }
            _ => (),
        }
    }

    if let Some(child) = children.last() {
        match child {
            TemplateNode::Text(t) if trim_after => {
                t.data = t.data.trim_end().to_string();
                if t.data.is_empty() {
                    block.pop_children();
                }
            }
            _ => (),
        }
    }

    // TODO: implement check else and elseif here
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

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::interfaces::{BaseNode, Text};
    use std::borrow::Borrow;
    use std::rc::Rc;

    #[test]
    fn test_trim_whitespace() {
        let sample = Rc::new(RefCell::new(TemplateNode::Text(Text {
            base_node: BaseNode {
                start: 0,
                end: 0,
                node_type: "Text".to_string(),
                children: Rc::new(RefCell::new(vec![Rc::new(RefCell::new(
                    TemplateNode::Text(Text::new("   Hello ".to_string())),
                ))])),
                prop_name: Default::default(),
                else_if: false,
                expression: None,
            },
            data: Rc::new(RefCell::new(" Hello ".to_string())),
        })));
        trim_whitespace(sample.clone(), true, false);
        let node = sample.borrow_mut();
        let children = node.children();
        let data = children
            .borrow_mut()
            .first()
            .unwrap()
            .borrow_mut()
            .get_data()
            .borrow_mut()
            .to_string();
        assert_eq!(data, "Hello ")
    }
}
