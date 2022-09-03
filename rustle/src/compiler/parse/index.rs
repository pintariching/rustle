use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;
use std::str::from_utf8;

use crate::compiler::interfaces::{
    Ast, BaseNode, Fragment, ParserOptions, Script, Style, TemplateNode, TmpNode,
};
use crate::compiler::parse::state::fragment;
use crate::compiler::utils::{full_char_at, WHITESPACE};
use crate::compiler::utils::{CompileError, NewErrorProps};
use regex::Regex;
use swc_ecma_ast::Ident;

use super::errors::Error;

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

//Return type of tag, mustache and text function corresponds to (ParserState | void) in svelte/src/compiler/parse/index.ts
pub enum StateReturn {
    Ok(ParserState),
    None,
}

//A function pointer for state
pub type ParserState = fn(&mut Parser) -> StateReturn;

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
            stack: Vec::new(),
            html: Fragment {
                base_node: BaseNode {
                    start: Some(0),
                    end: Some(0),
                    node_type: "Fragment".to_string(),
                    children: Vec::new(),
                    prop_name: HashMap::new(),
                    expression: None,
                },
            },
            css: Vec::new(),
            js: Vec::new(),
            meta_tags: Vec::new(),
            last_auto_closed_tag: None,
        };

        parser
            .stack
            .push(TemplateNode::BaseNode(parser.html.base_node.clone()));

        // Html is a Fragment but gets pushed to
        // parser.stack which is a Vec<TemplateNode> ??
        //parser.stack.push(parser.html);

        // fragment is a function
        // defined in src/compiler/parse/state/fragment.ts
        let mut state: ParserState = fragment;

        while parser.index < parser.template.len() {
            state = match state(&mut parser) {
                StateReturn::Ok(s) => s,
                StateReturn::None => fragment,
            };
        }

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

        //If the functions are identical their addresses should be too
        if state as usize != fragment as usize {
            parser.error("unexpected-eof", "Unexpected end of input")
        }

        if parser.html.base_node.children.len() > 0 {
            let mut start = parser.html.base_node.get_children()[0]
                .unwrap()
                .get_base_node()
                .start
                .unwrap();

            while WHITESPACE.is_match(from_utf8(&[template.as_bytes()[start]]).unwrap()) {
                start += 1;
            }

            let last = parser.html.base_node.get_children().len() - 1;
            let mut end = parser.html.base_node.get_children()[last]
                .unwrap()
                .get_base_node()
                .end
                .unwrap();

            while WHITESPACE.is_match(from_utf8(&[template.as_bytes()[end - 1]]).unwrap()) {
                end -= 1;
            }

            parser.html.base_node.start = Some(start);
            parser.html.base_node.end = Some(end);
        } else {
            parser.html.base_node.start = None;
            parser.html.base_node.end = None;
        }

        parser
    }

    pub fn current(&mut self) -> &mut TemplateNode {
        let length = self.stack.len() - 1;
        &mut self.stack[length]
    }

    pub fn error(&self, code: &str, message: &str) {
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

    pub fn eat(&mut self, str: &str, required: bool, error: Option<Error>) -> bool {
        if self.match_str(str) {
            self.index += str.len();
            return true;
        }

        if required {
            let e: Error;
            if let Some(err) = error {
                e = err
            } else {
                if self.index == self.template.len() {
                    e = Error::unexpected_eof_token(str);
                } else {
                    e = Error::unexpected_token(str)
                }
            }
            self.error(&e.code, &e.message)
        }

        false
    }

    // called "match" in the svelte parser
    pub fn match_str(&self, str: &str) -> bool {
        &self.template[self.index as usize..self.index as usize + str.len()] == str
    }

    pub fn match_regex(&self, pattern: Regex) -> Option<String> {
        let matches = pattern
            .find_iter(&self.template[self.index..self.template.len()])
            .map(|m| m.as_str().to_owned())
            .collect::<Vec<String>>();

        if matches.is_empty() {
            return None;
        }

        Some(matches[0].clone())
    }

    pub fn allow_whitespace(&mut self) {
        while self.index < self.template.len()
            && WHITESPACE.is_match(&self.template[self.index..self.template.len()])
        {
            self.index += 1
        }
    }

    pub fn read(&mut self, pattern: Regex) -> Option<String> {
        let result = self.match_regex(pattern);

        if let Some(r) = result.clone() {
            self.index += r.len();
        }

        result
    }

    pub fn read_identifier(&self, allow_reserved: Option<bool>) -> Option<String> {
        let start = self.index;
        let i = self.index;

        let code = full_char_at(&self.template, i);

        if !Ident::is_valid_start(code) {
            return None;
        }

        // Javascript magic?
        // i += code <= 0xffff ? 1 : 2;

        while i < self.template.len() {
            let code = full_char_at(&self.template, i);

            // TODO: find replacement for acorn/isIdentifierChar
            //if (!isIdentifierChar(code, true)) break;

            // More magic?
            // i += code <= 0xffff ? 1 : 2;
        }

        // what does (this.index = i) mean?
        // const identifier = this.template.slice(this.index, (this.index = i));
        // let identifier = self.template[self.index, ?];

        // if (!allow_reserved && reserved.has(identifier)) {
        // 	this.error(
        // 		{
        // 			code: "unexpected-reserved-word",
        // 			message: `'${identifier}' is a reserved word in JavaScript and cannot be used here`,
        // 		},
        // 		start
        // 	);
        // }

        // return identifier;

        todo!()
    }
}

pub fn parse(template: String, options: ParserOptions) -> Ast {
    let parser = Parser::new(template, options);

    // TODO we may want to allow multiple <style> tags —
    // one scoped, one global. for now, only allow one
    if parser.css.len() > 1 {
        parser.error(
            "Duplicate style",
            &parser.css[1].base_node.start.unwrap().to_string(),
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
            &instance_scripts[1].base_node.start.unwrap().to_string(),
        )
    }

    if module_scripts.len() > 1 {
        parser.error(
            "Duplicate module script",
            &module_scripts[1].base_node.start.unwrap().to_string(),
        )
    }

    Ast::new(
        parser.html,
        Some(parser.css[0].clone()),
        Some(instance_scripts[0].clone()),
        Some(module_scripts[0].clone()),
    )
}
