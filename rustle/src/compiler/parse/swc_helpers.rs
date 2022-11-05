use swc_common::source_map::Pos;
use swc_common::sync::Lrc;
use swc_common::{BytePos, FileName, SourceMap};
use swc_css_ast::Stylesheet;
use swc_css_parser::parse_str;
use swc_css_parser::parser::ParserConfig;
use swc_ecma_ast::{EsVersion, Expr, Program};
use swc_ecma_parser::Syntax;
use swc_ecma_parser::{parse_file_as_expr, parse_file_as_program};

use super::parser::Parser;

/// Parser the provided string using `swc_ecma_parser::parse_file_as_script` and returns
/// a `swc_ecma_ast::Script`.
///
/// # Arguments
///
/// * `source` - The Javascript string to parse
///
pub fn swc_parse_javascript(source: &str) -> Program {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Anon, source.into());

    let result = parse_file_as_program(
        &fm,
        Syntax::Es(Default::default()),
        EsVersion::latest(),
        None,
        &mut Vec::new(),
    )
    .unwrap();

    result
}

/// Parses an expression at the given index
/// and advances the `index` of the parser
/// to the end of the parsed expression
/// and returns `swc_ecma_ast::Expr`.
///
/// # Arguments
///
/// * `parser` - The `Parser` struct with the content and index set to the start of the expression
pub fn parse_expression_at(parser: &mut Parser) -> Expr {
    let source = parser
        .content
        .chars()
        .skip(parser.index)
        .collect::<String>()
        .replace("\n", " ");

    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Anon, source);

    let expr = parse_file_as_expr(
        &fm,
        Syntax::Es(Default::default()),
        EsVersion::latest(),
        None,
        &mut Vec::new(),
    )
    .unwrap();

    parser.index += get_end_position(&expr) - 1;

    expr.unwrap_parens().clone()
}

/// Gets the absolute end position of an expression.
fn get_end_position(expr: &Expr) -> usize {
    match expr {
        Expr::This(e) => e.span.hi.0 as usize,
        Expr::Array(e) => e.span.hi.0 as usize,
        Expr::Object(e) => e.span.hi.0 as usize,
        Expr::Fn(e) => e.function.span.hi.0 as usize,
        Expr::Unary(e) => e.span.hi.0 as usize,
        Expr::Update(e) => e.span.hi.0 as usize,
        Expr::Bin(e) => e.span.hi.0 as usize,
        Expr::Assign(e) => e.span.hi.0 as usize,
        Expr::Member(e) => e.span.hi.0 as usize,
        Expr::SuperProp(e) => e.span.hi.0 as usize,
        Expr::Cond(e) => e.span.hi.0 as usize,
        Expr::Call(e) => e.span.hi.0 as usize,
        Expr::New(e) => e.span.hi.0 as usize,
        Expr::Seq(e) => e.span.hi.0 as usize,
        Expr::Ident(e) => e.span.hi.0 as usize,
        Expr::Lit(e) => match e {
            swc_ecma_ast::Lit::Str(lit) => lit.span.hi.0 as usize,
            swc_ecma_ast::Lit::Bool(lit) => lit.span.hi.0 as usize,
            swc_ecma_ast::Lit::Null(lit) => lit.span.hi.0 as usize,
            swc_ecma_ast::Lit::Num(lit) => lit.span.hi.0 as usize,
            swc_ecma_ast::Lit::BigInt(lit) => lit.span.hi.0 as usize,
            swc_ecma_ast::Lit::Regex(lit) => lit.span.hi.0 as usize,
            swc_ecma_ast::Lit::JSXText(lit) => lit.span.hi.0 as usize,
        },
        Expr::Tpl(e) => e.span.hi.0 as usize,
        Expr::TaggedTpl(e) => e.span.hi.0 as usize,
        Expr::Arrow(e) => e.span.hi.0 as usize,
        Expr::Class(e) => e.class.span.hi.0 as usize,
        Expr::Yield(e) => e.span.hi.0 as usize,
        Expr::MetaProp(e) => e.span.hi.0 as usize,
        Expr::Await(e) => e.span.hi.0 as usize,
        Expr::Paren(e) => e.span.hi.0 as usize,
        Expr::JSXMember(e) => e.prop.span.hi.0 as usize,
        Expr::JSXNamespacedName(e) => e.name.span.hi.0 as usize,
        Expr::JSXEmpty(e) => e.span.hi.0 as usize,
        Expr::JSXElement(e) => e.span.hi.0 as usize,
        Expr::JSXFragment(e) => e.span.hi.0 as usize,
        Expr::TsTypeAssertion(e) => e.span.hi.0 as usize,
        Expr::TsConstAssertion(e) => e.span.hi.0 as usize,
        Expr::TsNonNull(e) => e.span.hi.0 as usize,
        Expr::TsAs(e) => e.span.hi.0 as usize,
        Expr::TsInstantiation(e) => e.span.hi.0 as usize,
        Expr::TsSatisfaction(e) => e.span.hi.0 as usize,
        Expr::PrivateName(e) => e.span.hi.0 as usize,
        Expr::OptChain(e) => e.span.hi.0 as usize,
        Expr::Invalid(e) => e.span.hi.0 as usize,
    }
}

pub fn swc_parse_css(source: &str) -> Stylesheet {
    let config = ParserConfig {
        allow_wrong_line_comments: false,
        css_modules: false,
        legacy_nesting: false,
    };

    let result = parse_str::<Stylesheet>(
        source,
        BytePos::from_usize(0),
        BytePos::from_usize(source.len()),
        config,
        &mut vec![],
    )
    .unwrap();

    result
}
