use swc_ecma_ast::Expr;

use self::{
    extract_variables::extract_root_variables,
    extract_variables_that_change::extract_variables_that_change,
};

use super::{expr_visitor::Visit, Fragment, RustleAst};
use std::collections::HashSet;

mod extract_variables;
mod extract_variables_that_change;

#[derive(Debug)]
pub struct AnalysisResult {
    pub variables: HashSet<String>,
    pub will_change: HashSet<String>,
    pub will_use_in_template: HashSet<String>,
}

pub fn analyse(ast: &RustleAst) -> AnalysisResult {
    let variables = extract_root_variables(&ast.script);
    let will_change = extract_variables_that_change(&ast.script);

    let mut will_use_in_template = Vec::new();
    for fragment in &ast.fragments {
        let mut used_variables = traverse_fragment(fragment);
        will_use_in_template.append(&mut used_variables);
    }

    AnalysisResult {
        variables: HashSet::from_iter(variables),
        will_change: HashSet::from_iter(will_change),
        will_use_in_template: HashSet::from_iter(will_use_in_template),
    }
}

fn traverse_fragment(fragment: &Fragment) -> Vec<String> {
    let mut will_use = Vec::new();
    match fragment {
        Fragment::Script(_) => (),
        Fragment::Element(f) => {
            for child in &f.fragments {
                let mut child_vars = traverse_fragment(child);
                will_use.append(&mut child_vars);
            }

            for attr in &f.attributes {
                match &attr.value {
                    Expr::Ident(ident) => will_use.push(ident.sym.to_string()),
                    _ => (),
                }
            }
        }
        Fragment::Expression(expr) => {
            will_use.append(&mut expr.extract_names());
        }
        Fragment::Text(_) => (),
    }

    will_use
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::compiler::parse::Parser;

    use super::analyse;

    #[test]
    fn test_analyse() {
        let source = fs::read_to_string("./tests/app.rustle").unwrap();
        let ast = Parser::new(&source).parse();
        let result = analyse(&ast);
    }
}
