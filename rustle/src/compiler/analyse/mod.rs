use swc_ecma_ast::{Expr, Stmt};

use self::{
    extract_root_variables::extract_root_variables,
    extract_variables_that_change::extract_variables_that_change,
};

use super::{expr_visitor::Visit, AttributeValue, Fragment, RustleAst};
use std::{cmp::Ordering, collections::HashSet};

mod extract_root_variables;
mod extract_variables_that_change;

#[derive(Debug)]
pub struct AnalysisResult {
    pub variables: HashSet<String>,
    pub will_change: HashSet<String>,
    pub will_use_in_template: HashSet<String>,
    pub reactive_declarations: Vec<ReactiveDeclaration>,
}

#[derive(Debug)]
pub struct ReactiveDeclaration {
    pub assignees: Vec<String>,
    pub dependencies: Vec<String>,
    pub node: Expr,
}

/// Analyses the AST and removes reactive declarations from the `RustleAst` script
pub fn analyse(ast: &mut RustleAst) -> AnalysisResult {
    let (root_variables, mut reactive_declarations) = extract_root_variables(&ast.script);
    let will_change = extract_variables_that_change(&ast.script);

    // remove labeled statements from the script body
    // as we add them later as regular variable declarations
    ast.script.body = ast
        .script
        .body
        .clone()
        .into_iter()
        .filter_map(|stmt| match stmt {
            Stmt::Labeled(_) => None,
            s => Some(s),
        })
        .collect::<Vec<Stmt>>();

    let mut will_use_in_template = Vec::new();
    for fragment in &ast.fragments {
        let mut used_variables = traverse_fragment(fragment);
        will_use_in_template.append(&mut used_variables);
    }

    // sort reactive declarations, so that they update in the right order
    // if they are written like this:
    //
    // $: quadruple = double * 2;
    // $: double = counter * 2;
    //
    // the updateReactiveDeclarations() function gets called only once and
    // quadruple isn't updated
    reactive_declarations.sort_by(|rd1, rd2| {
        // rd2 depends on what rd1 changes
        if rd1.dependencies.iter().any(|d| rd2.assignees.contains(d)) {
            // rd2 shoud come after rd1
            return Ordering::Greater;
        }

        // rd1 depends on what rd2 changes
        if rd2.dependencies.iter().any(|d| rd1.assignees.contains(d)) {
            // rd1 shoud come after rd2
            return Ordering::Less;
        }

        // don't change ordering
        Ordering::Less
    });

    AnalysisResult {
        variables: HashSet::from_iter(root_variables),
        will_change: HashSet::from_iter(will_change),
        will_use_in_template: HashSet::from_iter(will_use_in_template),
        reactive_declarations: reactive_declarations,
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
                    AttributeValue::Expr(expr) => match expr {
                        Expr::Ident(ident) => will_use.push(ident.sym.to_string()),
                        _ => (),
                    },
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
        let source = fs::read_to_string("./tests/app.svelte").unwrap();
        let mut ast = Parser::new(&source).parse();
        let result = analyse(&mut ast);
    }
}
