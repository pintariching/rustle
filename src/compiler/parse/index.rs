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

pub struct LastAutoClosedTag {
    pub tag: String,
    pub reason: String,
    pub depth: i32,
}

pub struct Parser {
    pub template: String,
    pub filename: Option<String>,
    pub custom_element: bool,
    pub index: i32,
    pub stack: Vec<TemplateNode>,
    pub html: Fragment,
    pub css: Vec<Style>,
    pub js: Vec<Script>,
    pub meta_tags: Vec<String>,
    pub last_auto_closed_tag: Option<LastAutoClosedTag>,
}

impl Parser {
    fn new(template: String, options: ParserOptions) -> Parser {
        let mut parser = Parser {
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

        // parser.stack.push(parser.html);

        // let state: ParserState = fragment;

        // while parser.index < parser.template.len() {
        //     state = state(parser);
        // }

        // TODO rewrite the constructor

        parser
    }

    // called "match" in the svelte parser
    pub fn match_str(&self, str: &str) -> bool {
        &self.template[self.index as usize..self.index as usize + str.len()] == str
    }
}
