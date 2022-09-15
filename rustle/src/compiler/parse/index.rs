use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::Index;
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
    pub meta_tags: HashMap<String, bool>,
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
                    elseif: false,
                    _else: false,
                },
            },
            css: Vec::new(),
            js: Vec::new(),
            meta_tags: HashMap::new(),
            last_auto_closed_tag: None,
        };

        parser
            .stack
            .push(TemplateNode::BaseNode(parser.html.base_node.clone()));

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
                    current_type = format!("<{}>", e.name.clone());
                    current_slug = "element".to_string();
                }
                _ => {
                    current_type = "Block".to_string();
                    current_slug = "block".to_string();
                }
            }

            // panics
            parser.error(
                &format!("unclosed-{}", current_slug),
                &format!("{} was left open", current_type),
                None,
            );
        }

        //If the functions are identical their addresses should be too
        if state as usize != fragment as usize {
            parser.error("unexpected-eof", "Unexpected end of input", None)
        }

        if parser.html.base_node.children.len() > 0 {
            let mut start = parser
                .html
                .base_node
                .get_children()
                .iter_mut()
                .nth(0)
                .unwrap()
                .get_base_node()
                .start
                .unwrap();

            while WHITESPACE.is_match(template.chars().nth(start).unwrap().to_string().as_str()) {
                start += 1;
            }

            let last_index = parser.html.base_node.get_children().len() - 1;
            let mut end = parser
                .html
                .base_node
                .get_children()
                .iter_mut()
                .nth(last_index)
                .unwrap()
                .get_base_node()
                .end
                .unwrap();

            while WHITESPACE.is_match(template.chars().nth(end - 1).unwrap().to_string().as_str()) {
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
        self.stack.iter_mut().nth(length).unwrap()
    }

    pub fn error(&self, code: &str, message: &str, index: Option<usize>) {
        let i = match index {
            Some(i) => i,
            None => self.index,
        };

        let error = NewErrorProps {
            name: "ParseError",
            code,
            source: &self.template,
            start: i,
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
            self.error(&e.code, &e.message, None)
        }

        false
    }

    // called "match" in the svelte parser
    pub fn match_str(&self, str: &str) -> bool {
        &self.template[self.index..self.index + str.len()] == str
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
            && WHITESPACE.is_match(
                self.template
                    .chars()
                    .nth(self.index)
                    .unwrap()
                    .to_string()
                    .as_str(),
            )
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
        let mut i = self.index;

        let code = full_char_at(&self.template, i);

        if !Ident::is_valid_start(code) {
            return None;
        }

        // 0xffff = 65535 == u16::MAX
        if code as u16 <= u16::MAX {
            i += 1;
        } else {
            i += 2;
        }

        while i < self.template.len() {
            let code = full_char_at(&self.template, i);

            if Ident::is_valid_continue(code) {
                break;
            }

            // 0xffff = 65535 == u16::MAX
            if code as u16 <= u16::MAX {
                i += 1;
            } else {
                i += 2;
            }
        }

        // what does (this.index = i) mean?
        // const identifier = this.template.slice(this.index, (this.index = i));
        //let identifier = self.template[self.index..self.index = i];

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

    pub fn read_until(&mut self, pattern: Regex, error: Option<Error>) -> String {
        if self.index >= self.template.len() {
            if let Some(error) = error {
                self.error(&error.code, &error.message, None);
            } else {
                self.error("unexpected-eof", "Unexpected end of input", None);
            }
        }

        let start = self.index;
        let matches = pattern.find(&self.template[start..]);

        if let Some(m) = matches {
            self.index = start + m.start();
            return self.template[start..self.index].to_string();
        }

        self.index = self.template.len();
        return self.template[start..].to_string();
    }

    pub fn require_whitespace(&mut self) {
        let c = self.template.chars().nth(self.index).unwrap().to_string();
        if WHITESPACE.is_match(&c) {
            self.error("missing-whitespace", "Expected whitespace", None);
        }

        self.allow_whitespace();
    }
}

pub fn parse(template: String, options: ParserOptions) -> Ast {
    let parser = Parser::new(template, options);

    // TODO we may want to allow multiple <style> tags —
    // one scoped, one global. for now, only allow one
    if parser.css.len() > 1 {
        let error = Error::duplicate_style();
        parser.error(
            &error.code,
            &error.message,
            parser.css.iter().nth(1).unwrap().base_node.start,
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
        let error = Error::invalid_script_instance();
        parser.error(
            &error.code,
            &error.message,
            instance_scripts.iter().nth(1).unwrap().base_node.start,
        )
    }

    if module_scripts.len() > 1 {
        let error = Error::invalid_script_module();
        parser.error(
            &error.code,
            &error.message,
            module_scripts.iter().nth(1).unwrap().base_node.start,
        )
    }

    Ast::new(
        parser.html,
        Some(parser.css[0].clone()),
        Some(instance_scripts[0].clone()),
        Some(module_scripts[0].clone()),
    )
}
