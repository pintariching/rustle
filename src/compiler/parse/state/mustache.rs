use crate::compiler::interfaces::{TemplateNode, TmpNode};

pub fn trim_whitespace(block: &mut TemplateNode, trim_before: bool, trim_after: bool) {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::interfaces::{BaseNode, Text};
    use std::collections::HashMap;

    #[test]
    fn test_trim_whitespace() {
        let mut sample = TemplateNode::Text(Text {
            base_node: BaseNode {
                start: 0,
                end: 0,
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
}
