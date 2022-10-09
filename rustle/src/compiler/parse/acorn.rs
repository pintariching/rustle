use crate::compiler::node::Node;
use rustle_code_red::{parse as code_red_parse, parse_expression_at as code_red_parse_expression_at};
use swc_ecma_ast::{Program, Expr};

pub fn parse(source: String) -> Program {
    let program = code_red_parse(source).unwrap();

    program
}

pub fn parse_expression_at(source: String, index: usize) -> Box<Expr> {
    let expr = code_red_parse_expression_at(source, index).unwrap();

   expr
}
