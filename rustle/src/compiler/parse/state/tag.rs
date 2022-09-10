use crate::compiler::{
    interfaces::TemplateNode,
    parse::index::{Parser, StateReturn},
};
use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VALID_TAG_NAME: Regex = Regex::new(r"/^\!?[a-zA-Z]{1,}:?[a-zA-Z0-9\-]*/").unwrap();
    static ref META_TAGS: HashMap<&'static str, &'static str> = HashMap::from([
        ("svelte:head", "Head"),
        ("svelte:options", "Options"),
        ("svelte:window", "Window"),
        ("svelte:body", "Body")
    ]);
    static ref SELF: Regex = Regex::new(r"/^svelte:self(?=[\s/>])/").unwrap();
    static ref COMPONENT: Regex = Regex::new(r"/^svelte:component(?=[\s/>])/").unwrap();
    static ref SLOT: Regex = Regex::new(r"/^svelte:fragment(?=[\s/>])/").unwrap();
    static ref ELEMENT: Regex = Regex::new(r"/^svelte:element(?=[\s/>])/").unwrap();
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
    let parent = parser.current();

    if parser.eat("!--", false, None) {
        //let data = parser.read_until("-->");
    }

    todo!()
}
