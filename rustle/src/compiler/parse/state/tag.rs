use crate::{
    compiler::{
        compile::utils::extract_svelte_ignore::extract_svelte_ignore,
        interfaces::{
            Attribute, BaseNode, Comment, DirectiveType, Script, Special, Style, TemplateNode, Text,
        },
        node::Node,
        parse::{
            errors::Error,
            index::{LastAutoClosedTag, Parser, StateReturn},
            read::{read_expression, read_script, read_style},
            utils::closing_tag_omitted,
        },
    },
    shared::is_void,
};
use std::{collections::HashMap, hash::Hash, ops::Index};

use lazy_static::lazy_static;
use regex::Regex;
use swc_estree_ast::{BaseNode as SwcBaseNode, Identifier};

lazy_static! {
    static ref VALID_TAG_NAME: Regex = Regex::new("^!?[a-zA-Z]{1,}:?[a-zA-Z0-9-]*").unwrap();
    static ref META_TAGS: HashMap<&'static str, &'static str> = HashMap::from([
        ("svelte:head", "Head"),
        ("svelte:options", "Options"),
        ("svelte:window", "Window"),
        ("svelte:body", "Body")
    ]);

    /// `regex` doesn't support lookahead so make sure to remove
    /// the last character
    static ref SELF: Regex = Regex::new("^svelte:self([\\s/>])").unwrap();

    /// `regex` doesn't support lookahead so make sure to remove
    /// the last character
    static ref COMPONENT: Regex = Regex::new("^svelte:component([\\s/>])").unwrap();

    /// `regex` doesn't support lookahead so make sure to remove
    /// the last character
    static ref SLOT: Regex = Regex::new("^svelte:fragment([\\s/>])").unwrap();

    /// `regex` doesn't support lookahead so make sure to remove
    /// the last character
    static ref ELEMENT: Regex = Regex::new("^svelte:element([\\s/>])").unwrap();

    static ref SPECIALS: HashMap<&'static str, Specials> = HashMap::from([
        ("script",
        Specials {
            read: read_script,
            property: "js"
        }),
        ("style",
        Specials {
            read: read_style,
            property: "css"
        })
    ]);
}

struct Specials {
    read: fn(&mut Parser, usize, Vec<TemplateNode>) -> Special,
    property: &'static str,
}

pub const VALID_META_TAGS: [&'static str; 4] = [
    "svelte:self",
    "svelte:component",
    "svelte:fragment",
    "svelte:element",
];

pub fn parent_is_head(stack: Vec<TemplateNode>) -> bool {
    for i in (0..stack.len()).rev() {
        if let Some(temp) = stack.iter().nth(i) {
            let temp_type = &temp.get_type();

            if temp_type == "Head" {
                return true;
            }
            if temp_type == "Element" || temp_type == "InlineComponent" {
                return false;
            }
        }
    }

    false
}

pub fn tag(parser: &mut Parser) -> StateReturn {
    let start = parser.index + 1;

    if parser.eat("!--", false, None) {
        let data = parser.read_until(Regex::new("-->").unwrap(), None);
        parser.eat("-->", true, Some(Error::unclosed_comment()));

        let index = parser.index;
        parser
            .current()
            .get_children()
            .push(TemplateNode::Comment(Comment {
                base_node: BaseNode {
                    start: Some(start),
                    end: Some(index),
                    node_type: "Comment".to_string(),
                    children: Vec::new(),
                    prop_name: HashMap::new(),
                    expression: None,
                    elseif: false,
                    _else: false,
                },
                data: data.clone(),
                ignores: extract_svelte_ignore(&data),
            }));

        return StateReturn::None;
    }

    let is_closing_tag = parser.eat("/", false, None);
    let name = read_tag_name(parser);
    let name = name.as_str();

    if META_TAGS.contains_key(name) {
        let slug = META_TAGS.get(name).unwrap().to_lowercase();
        if is_closing_tag {
            if (name == "svelte:window" || name == "svelte:body")
                && parser.current().get_children().len() > 0
            {
                let error = Error::invalid_element_content(&slug, &name);
                let index = parser
                    .current()
                    .get_children()
                    .first_mut()
                    .unwrap()
                    .get_base_node()
                    .start;
                parser.error(&error.code, &error.message, index);
            } else {
                if parser.meta_tags.contains_key(name) {
                    let error = Error::duplicate_element(&slug, &name);
                    parser.error(&error.code, &error.message, Some(start));
                }

                if parser.stack.len() > 1 {
                    let error = Error::invalid_element_placement(&slug, &name);
                    parser.error(&error.code, &error.message, Some(start));
                }

                parser.meta_tags.insert(name.to_string(), true);
            }
        }
    }

    let fragment_type: String = match META_TAGS.get(name) {
        Some(f) => f.to_string(),
        None => {
            if Regex::new("[A-Z]")
                .unwrap()
                .is_match(name.chars().nth(0).unwrap().to_string().as_str())
                || name == "svelte:self"
                || name == "svelte:component"
            {
                "InlineComponent".to_string()
            } else if name == "svelte:fragment" {
                "SlotTemplate".to_string()
            } else if name == "title" && parent_is_head(parser.stack.clone()) {
                "Title".to_string()
            } else if name == "slot" && !parser.custom_element {
                "Slot".to_string()
            } else {
                "Element".to_string()
            }
        }
    };

    let element = TemplateNode::Attribute(Attribute {
        base_node: BaseNode {
            start: Some(start),
            end: None,
            node_type: fragment_type.to_string(),
            children: Vec::new(),
            prop_name: HashMap::new(),
            expression: None,
            elseif: false,
            _else: false,
        },
        name: name.to_string(),
        value: Vec::new(),
    });

    parser.allow_whitespace();

    if is_closing_tag {
        if is_void(name) {
            let error = Error::invalid_void_content(name);
            parser.error(&error.code, &error.message, Some(start));
        }

        parser.eat(">", true, None);

        // close any elements that don't have their own closing tags, e.g. <div><p></div>
        while parser.current().get_name().unwrap().as_str() != name {
            if parser.current().get_type() != "Element" {
                let error = if let Some(last_auto_closed_tag) = &parser.last_auto_closed_tag {
                    if last_auto_closed_tag.tag.as_str() == name {
                        Error::invalid_closing_tag_autoclosed(name, &last_auto_closed_tag.reason)
                    } else {
                        Error::invalid_closing_tag_unopened(name)
                    }
                } else {
                    Error::invalid_closing_tag_unopened(name)
                };
                parser.error(&error.code, &error.message, Some(start));
            }

            parser.current().get_base_node().end = Some(start);
            parser.stack.pop();

            if let Some(last_auto_closed_tag) = &parser.last_auto_closed_tag {
                if parser.stack.len() < last_auto_closed_tag.depth {
                    parser.last_auto_closed_tag = None;
                }
            }
        }

        return StateReturn::None;
    } else if closing_tag_omitted(parser.current().get_name().unwrap().as_str(), Some(name)) {
        parser.current().get_base_node().end = Some(start);
        parser.stack.pop();
        parser.last_auto_closed_tag = Some(LastAutoClosedTag {
            tag: parser.current().get_name().unwrap(),
            reason: name.to_string(),
            depth: parser.stack.len(),
        })
    }

    let unique_names: Vec<String> = Vec::new();
    let attribute: TemplateNode;

    // TODO: implement read_attribute and continue on line 166

    todo!()
}

fn read_tag_name(parser: &mut Parser) -> String {
    todo!()
}

fn check_unique(parser: &mut Parser, name: &str, mut unique_names: Vec<String>, start: usize) {
    if unique_names.contains(&name.to_string()) {
        let error = Error::duplicate_attribute();
        parser.error(&error.code, &error.message, Some(start))
    }
    unique_names.push(name.to_string());
}

fn read_attribute(parser: &mut Parser, mut unique_names: Vec<String>) -> Option<TemplateNode> {
    let start = parser.index;

    // reads curly brace expressions?
    if parser.eat("{", false, None) {
        parser.allow_whitespace();

        if parser.eat("...", false, None) {
            let expression = read_expression(parser);

            parser.allow_whitespace();
            parser.eat("}", true, None);

            return Some(TemplateNode::BaseNode(BaseNode {
                start: Some(start),
                end: Some(parser.index),
                node_type: "Spread".to_string(),
                children: Vec::new(),
                prop_name: HashMap::new(),
                expression: Some(expression),
                elseif: false,
                _else: false,
            }));
        } else {
            let value_start = parser.index;

            let name = parser.read_identifier(None);
            parser.allow_whitespace();
            parser.eat("}", true, None);

            if let Some(name) = name {
                check_unique(parser, name.as_str(), unique_names, start);

                return Some(TemplateNode::Attribute(Attribute {
                    base_node: BaseNode {
                        start: Some(start),
                        end: Some(parser.index),
                        node_type: "Attribute".to_string(),
                        children: Vec::new(),
                        prop_name: HashMap::new(),
                        expression: None,
                        elseif: false,
                        _else: false,
                    },
                    name: name.clone(),
                    value: vec![TemplateNode::BaseNode(BaseNode {
                        start: Some(value_start),
                        end: Some(value_start + name.len()),
                        node_type: "Attribute".to_string(),
                        children: Vec::new(),
                        prop_name: HashMap::new(),
                        expression: Some(Node::Identifier(Identifier {
                            base: SwcBaseNode {
                                start: Some(value_start as u32),
                                end: Some((value_start + name.len()) as u32),
                                leading_comments: Vec::new(),
                                inner_comments: Vec::new(),
                                trailing_comments: Vec::new(),
                                range: None,
                                loc: None,
                            },
                            name: name.into(),
                            decorators: None,
                            optional: None,
                            type_annotation: None,
                        })),
                        elseif: false,
                        _else: false,
                    })],
                }));
            } else {
                let error = Error::empty_attribute_shorthand();
                parser.error(&error.code, &error.message, Some(start));
            }

            // eslint-disable-next-line no-useless-escape
            let name = parser.read_until(Regex::new("[s=/>\"\'").unwrap(), None);
            if name.is_empty() {
                return None;
            }

            let end = parser.index;

            parser.allow_whitespace();

            let colon_index = name.find(':').unwrap();
            let directive_type = get_directive_type(&name[..colon_index]);

            let value = true;

            if parser.eat("=", false, None) {
                parser.allow_whitespace();
                //value = read_attribute(parser, unique_names)
                // TODO: implement read_attribute_value
            }
        }
    }

    todo!()
}

fn get_directive_type(name: &str) -> DirectiveType {
    match name {
        "use" => DirectiveType::Action,
        "animate" => DirectiveType::Animation,
        "bind" => DirectiveType::Binding,
        "class" => DirectiveType::Class,
        "style" => DirectiveType::StyleDirective,
        "on" => DirectiveType::EventHandler,
        "let" => DirectiveType::Let,
        "ref" => DirectiveType::Ref,
        "in" | "out" | "transition" => DirectiveType::Transition,
        _ => panic!("Incorrect directive type name"),
    }
}

fn read_attribute_value(parser: &mut Parser) -> Vec<Text> {
    let quote_mark = if parser.eat("'", false, None) {
        Some("'")
    } else if parser.eat(r#"""#, false, None) {
        Some(r#"""#)
    } else {
        None
    };

    if let Some(mark) = quote_mark {
        if parser.eat(mark, false, None) {
            return vec![Text {
                base_node: BaseNode {
                    start: Some(parser.index - 1),
                    end: Some(parser.index - 1),
                    node_type: "Text".to_string(),
                    children: Vec::new(),
                    prop_name: HashMap::new(),
                    expression: None,
                    elseif: false,
                    _else: false,
                },
                data: String::new(),
                raw: String::new(),
            }];
        }
    }

    let regex = match quote_mark {
        Some("'") => Regex::new("'").unwrap(),
        Some(r#"""#) => Regex::new("\"").unwrap(),
        Some(_) | None => Regex::new("(/>|[s\"'=<>`])").unwrap(),
    };

    // TODO: implement read_sequence
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{COMPONENT, ELEMENT, SELF, SLOT, VALID_TAG_NAME};

    #[test]
    fn test_valid_tag_name_regex() {
        let samples = vec![
            "svelte:options",
            "svelte:component",
            "my-element",
            "svelte:my-element",
            "!svelte:element",
            "element-multiple-dashes",
            "!svelte:multiple-dashes-element",
        ];

        for s in samples {
            assert!(VALID_TAG_NAME.is_match(s));
        }
    }

    #[test]
    fn test_self_regex() {
        let samples = vec!["svelte:self ", "svelte:self/", "svelte:self>"];

        for s in samples {
            s.ends_with("/>");
            assert!(SELF.is_match(s));
        }
    }

    #[test]
    fn test_component_regex() {
        let samples = vec![
            "svelte:component ",
            "svelte:component/",
            "svelte:component>",
        ];

        for s in samples {
            s.ends_with("/>");
            assert!(COMPONENT.is_match(s));
        }
    }

    #[test]
    fn test_slot_regex() {
        let samples = vec!["svelte:fragment ", "svelte:fragment/", "svelte:fragment>"];

        for s in samples {
            s.ends_with("/>");
            assert!(SLOT.is_match(s));
        }
    }

    #[test]
    fn test_element_regex() {
        let samples = vec!["svelte:element ", "svelte:element/", "svelte:element>"];

        for s in samples {
            s.ends_with("/>");
            assert!(ELEMENT.is_match(s));
        }
    }
}
