use crate::compiler::parse::index::{Parser, StateReturn};
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

pub fn tag(parser: &mut Parser) -> StateReturn {
    todo!()
}
