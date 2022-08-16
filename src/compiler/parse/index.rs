use crate::compiler::{
    interfaces::{Ast, BaseNode, Fragment, ParserOptions, Script, Style, TemplateNode},
    utils::{CompileError, NewErrorProps},
};
use lazy_static::lazy_static;
use regex::Regex;

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
    pub index: usize,
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
            template: Regex::new(r"/\s+$/")
                .unwrap()
                .replace(&template, "")
                .to_string(),
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

        // Html is a Fragment but gets pushed to
        // parser.stack which is a Vec<TemplateNode> ??
        //parser.stack.push(parser.html);

        // fragment is a function
        // defined in src/compiler/parse/state/fragment.ts
        // let state: ParserState = fragment;

        // while parser.index < parser.template.len() {
        //     state = state(parser) || fragment;
        // }

        if parser.stack.len() > 1 {
            let current = parser.current();

            let mut current_type = String::new();
            let mut current_slug = String::new();

            match current {
                TemplateNode::Element(e) => {
                    current_type = e.name.clone();
                    current_slug = "element".to_string();
                }
                _ => current_slug = "block".to_string(),
            }

            // panics
            parser.error(
                &format!("unclosed-{}", current_slug),
                &format!("{} was left open", current_type),
            );
        }

        // TODO: rewrite this to rust
        // if (state !== fragment) {
        // 	this.error({
        // 		code: 'unexpected-eof',
        // 		message: 'Unexpected end of input'
        // 	});
        // }

        if let Some(children) = &parser.html.base_node.children {
            if children.len() > 0 {
                // TODO: impl BaseNodeTrait to get values from common base node?
                //let start = children[0].start;
            }
        }

        parser
    }

    fn current(&self) -> &TemplateNode {
        &self.stack[self.stack.len() - 1]
    }

    fn error(&self, code: &str, message: &str) {
        let error = NewErrorProps {
            name: "ParseError",
            code,
            source: &self.template,
            start: self.index,
            end: None,
            filename: &self.filename.clone().unwrap(),
        };

        let compile_error = CompileError::new(message, error);
        panic!("{:#?}", compile_error);
    }
    // called "match" in the svelte parser
    pub fn match_str(&self, str: &str) -> bool {
        &self.template[self.index as usize..self.index as usize + str.len()] == str
    }
}

pub fn parse(template: String, options: ParserOptions) -> Ast {
    let parser = Parser::new(template, options);

    // TODO we may want to allow multiple <style> tags —
    // one scoped, one global. for now, only allow one
    if parser.css.len() > 1 {
        parser.error(
            "Duplicate style",
            &parser.css[1].base_node.start.to_string(),
        );
    }

    let instance_scripts = parser
        .js
        .iter()
        .filter(|script| script.context == "default")
        .collect::<Vec<&Script>>();

    let module_scripts = parser
        .js
        .iter()
        .filter(|script| script.context == "module")
        .collect::<Vec<&Script>>();

    if instance_scripts.len() > 1 {
        parser.error(
            "Duplicate script",
            &instance_scripts[1].base_node.start.to_string(),
        )
    }

    if module_scripts.len() > 1 {
        parser.error(
            "Duplicate module script",
            &module_scripts[1].base_node.start.to_string(),
        )
    }

    Ast::new(
        parser.html,
        Some(parser.css[0].clone()),
        Some(instance_scripts[0].clone()),
        Some(module_scripts[0].clone()),
    )
}
