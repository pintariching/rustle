use crate::compiler::interfaces::{Children, GetChildren, TemplateNode, Text};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub fn trim_whitespace(
    block: Rc<RefCell<TemplateNode>>,
    trim_before: bool,
    trim_after: bool,
) -> Option<()> {
    let children = block.borrow().children();
    if children.borrow().is_empty() {
        return None;
    }

    let mut is_data_empty = false;
    if let Some(child) = children.borrow().first() {
        let mut new_data = "".to_string();
        if child.borrow().get_type() == "Text" && trim_before {
            let data = child.borrow().get_data();
            new_data = data.borrow().trim_start().to_string();
            data.replace(new_data.clone());
        }

        if new_data.is_empty() {
            is_data_empty = true;
        }
    }

    if is_data_empty {
        let mut data = children.borrow().clone();
        data.remove(0);
        children.replace(data);
        is_data_empty = false;
    }

    if let Some(child) = children.borrow().last() {
        let mut new_data = "".to_string();
        if child.borrow().get_type() == "Text" && trim_after {
            let data = child.borrow().get_data();
            new_data = data.borrow().trim_start().to_string();
            data.replace(new_data.clone());
        }

        if new_data.is_empty() {
            is_data_empty = true;
        }
    }

    if is_data_empty {
        let mut data = children.borrow().clone();
        data.pop();
        children.replace(data);
        is_data_empty = false;
    }

    // TODO: implement check else and elseif here

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
