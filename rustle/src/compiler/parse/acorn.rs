use crate::compiler::node::Node;
use swc_common::sync::Lrc;
use swc_common::{
    errors::{ColorConfig, Handler},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecma_ast::{Expr, Script};
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};

pub fn parse(source: String) -> Script {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.new_source_file(FileName::Custom(String::new()), source);

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

    swc_parser
        .parse_script()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parse script")
}

pub fn parse_expression_at(source: String, index: usize) -> Expr {
    let source = source
        .chars()
        .skip(index)
        .collect::<String>()
        .replace("\n", " ");

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.new_source_file(FileName::Custom(String::new()), source);

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

    swc_parser
        .parse_expr()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("failed to parse script")
        .unwrap_parens()
        .clone()
}
