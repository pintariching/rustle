use std::collections::HashMap;

use crate::compiler::{
    interfaces::{BaseNode, Script, TemplateNode},
    node::Node,
    parse::{errors::Error, index::Parser},
};
use lazy_static::lazy_static;
use regex::Regex;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};

lazy_static! {
    static ref SCRIPT_REGEX: Regex = Regex::new("</script\\s*>").unwrap();
}

fn get_context(parser: &mut Parser, attributes: Vec<TemplateNode>, start: usize) -> String {
    let context = attributes
        .iter()
        .find(|a| a.get_name().unwrap() == "context");

    if let Some(c) = context {
        match c {
            TemplateNode::Attribute(t) => {
                if t.value.len() != 1 || t.value[0].get_type() != "Text".to_string() {
                    let error = Error::invalid_script_context_attribute();
                    parser.error(&error.code, &error.message, Some(start));
                }

                let value = t.value[0].get_data();

                if value != "module".to_string() {
                    let error = Error::invalid_script_context_value();
                    parser.error(&error.code, &error.message, t.base_node.start);
                }

                return value;
            }
            _ => (),
        }
    } else {
        return "default".to_string();
    }

    todo!()
}

pub fn read_script(parser: &mut Parser, start: usize, attributes: Vec<TemplateNode>) -> Script {
    let script_start = parser.index;
    let data = parser.read_until(SCRIPT_REGEX.clone(), Some(Error::unclosed_script()));

    if parser.index >= parser.template.len() {
        let error = Error::unclosed_script();
        parser.error(&error.code, &error.message, None);
    }

    let mut source = parser
        .template
        .chars()
        .take(script_start)
        .collect::<String>()
        .replace("\n", " ");

    source.push_str(&data);

    parser.read(SCRIPT_REGEX.clone());

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.new_source_file(FileName::Custom(parser.filename.clone().unwrap()), source);

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut swc_parser = SwcParser::new_from(lexer);

    for e in swc_parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let program = swc_parser
        .parse_program()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parse module");

    return Script {
        base_node: BaseNode {
            node_type: "Script".to_string(),
            start: Some(start),
            end: Some(parser.index),
            children: Vec::new(),
            prop_name: HashMap::new(),
            expression: None,
            elseif: false,
            _else: false,
        },
        context: get_context(parser, attributes, start),
        content: program,
    };
}

#[cfg(test)]
mod tests {
    use crate::compiler::parse::index::Parser;

    use super::SCRIPT_REGEX;

    #[test]
    fn test_script_regex() {
        let samples = vec!["</script>", "</script >"];

        for s in samples {
            assert!(SCRIPT_REGEX.is_match(s));
        }
    }
}
