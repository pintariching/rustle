use crate::compiler::interfaces::{BaseNode, Fragment, ParserOptions, Script, Style, TemplateNode};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"/\s+$/").unwrap();
}

enum ParserState {
    Parser(Parser),
    Void,
}

struct LastAutoClosedTag {
    tag: String,
    reason: String,
    depth: i32,
}

pub struct Parser {
    template: String,
    filename: Option<String>,
    custom_element: bool,
    index: i32,
    stack: Vec<TemplateNode>,
    html: Fragment,
    css: Vec<Style>,
    js: Vec<Script>,
    meta_tags: Vec<String>,
    last_auto_closed_tag: Option<LastAutoClosedTag>,
}

impl Parser {
    fn new(template: String, options: ParserOptions) -> Parser {
        let parser = Parser {
            template: TEMPLATE_REGEX.replace(&template, "").to_string(),
            filename: options.filename,
            custom_element: options.custom_element,
            index: 0,
            stack: Vec::new(), // TODO: push html to stack
            html: Fragment {
                base_node: BaseNode {
                    start: 0,
                    end: 0,
                    node_type: "Fragment".to_string(),
                    children: Some(Vec::new()),
                    prop_name: Vec::new(),
                },
            },
            css: Vec::new(),
            js: Vec::new(),
            meta_tags: Vec::new(),
            last_auto_closed_tag: None,
        };

        // TODO rewrite the constructor

        parser
    }
}
