use rand::{distributions::Alphanumeric, Rng};
use swc_ecma_ast::{Expr, Stmt};

use self::{
    extract_css_classes::extract_css_classes, extract_root_variables::extract_root_variables,
    extract_variables_that_change::extract_variables_that_change,
};

use super::{expr_visitor::Visit, AttributeValue, Fragment, RustleAst};
use std::{cmp::Ordering, collections::HashSet};

mod extract_css_classes;
mod extract_root_variables;
mod extract_variables_that_change;

#[derive(Debug)]
pub struct AnalysisResult {
    pub will_change: HashSet<String>,
    pub will_use_in_template: HashSet<String>,
    pub reactive_declarations: Vec<ReactiveDeclaration>,
    pub variables: HashSet<String>,
    pub css_classes: HashSet<String>,
    pub css_classes_in_template: HashSet<String>,
    pub css_unique_scope: String,
}

#[derive(Debug)]
pub struct ReactiveDeclaration {
    pub assignees: Vec<String>,
    pub dependencies: Vec<String>,
    pub node: Expr,
}

/// Analyses the AST and removes reactive declarations from the `RustleAst` script
pub fn analyse(ast: &mut RustleAst) -> AnalysisResult {
    let mut root_variables = Vec::new();
    let mut reactive_declarations = Vec::new();
    let mut will_change = Vec::new();
    let mut will_use_in_template = Vec::new();
    let mut css_classes_in_template = Vec::new();

    if let Some(script) = &mut ast.script {
        (root_variables, reactive_declarations) = extract_root_variables(script);
        will_change = extract_variables_that_change(script);

        // remove labeled statements from the script body
        // as we add them later as regular variable declarations
        script.body = script
            .body
            .clone()
            .into_iter()
            .filter_map(|stmt| match stmt {
                Stmt::Labeled(_) => None,
                s => Some(s),
            })
            .collect::<Vec<Stmt>>();

        for fragment in &ast.fragments {
            let mut used_variables = extract_variables_in_template(fragment);
            will_use_in_template.append(&mut used_variables);

            let mut used_classes = extract_css_in_template(fragment);
            css_classes_in_template.append(&mut used_classes);
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
    }

    let mut css_classes = Vec::new();
    if let Some(style) = &mut ast.style {
        css_classes = extract_css_classes(style);
    }

    AnalysisResult {
        variables: HashSet::from_iter(root_variables),
        will_change: HashSet::from_iter(will_change),
        will_use_in_template: HashSet::from_iter(will_use_in_template),
        reactive_declarations: reactive_declarations,
        css_classes: HashSet::from_iter(css_classes),
        css_classes_in_template: HashSet::from_iter(css_classes_in_template),
        css_unique_scope: format!(
            "rustle-{}",
            rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect::<String>()
        ),
    }
}

fn extract_variables_in_template(fragment: &Fragment) -> Vec<String> {
    let mut will_use = Vec::new();
    match fragment {
        Fragment::Script(_) => (),
        Fragment::Element(f) => {
            for child in &f.fragments {
                let mut child_vars = extract_variables_in_template(child);
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
        _ => (),
    }

    will_use
}

fn extract_css_in_template(fragment: &Fragment) -> Vec<String> {
    let mut will_use = Vec::new();

    match fragment {
        Fragment::Element(e) => {
            for attr in &e.attributes {
                if &attr.name == "class" {
                    match &attr.value {
                        AttributeValue::String(s) => {
                            if !s.is_empty() {
                                let mut values =
                                    s.split(' ').map(String::from).collect::<Vec<String>>();
                                will_use.append(&mut values);
                            }
                        }
                        AttributeValue::Expr(_) => {
                            panic!("Attribute value shouldn't be an expression: {:#?}", e)
                        }
                    }
                }
            }
        }
        _ => (),
    }

    will_use
}
