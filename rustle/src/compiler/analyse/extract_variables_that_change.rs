use swc_ecma_ast::Script;

use crate::compiler::swc_ast_visitor::{Interpreter, Visitor};

pub fn extract_variables_that_change(script: &Script) -> Vec<String> {
    let body = script.body.clone();

    let mut interpreter = Interpreter;
    let vars = body
        .iter()
        .filter_map(|stmt| Interpreter::visit_stmt(&mut interpreter, stmt))
        .collect::<Vec<String>>();

    vars
}
