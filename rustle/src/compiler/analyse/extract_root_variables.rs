use swc_ecma_ast::{Decl, Expr, Pat, PatOrExpr, Script, Stmt};

use crate::compiler::expr_visitor::Visit;

use super::ReactiveDeclaration;

/// Extracts the root variables and reactive declarationsfrom a given `swc_ecma_ast::Script`
/// and returns them as a tuple.
///
/// Root variables are variables defined in the root scope, for example:
///
/// ```ignore
/// // javascript
/// let counter = 5;
/// let some_variable = "text";
/// $: quadruple = counter * 2;
///
/// function double(value) {
///     let two = 2;
///     value * 2
/// }
/// ```
/// the root variables are "counter" and "some_variable" and reactive declaration is "quadruple".
/// From the reactive declaration, the left and right side are extracted. In the above example
/// "quadruple" is put into `assignees` and "counter" put into `dependencies`.
pub fn extract_root_variables(script: &Script) -> (Vec<String>, Vec<ReactiveDeclaration>) {
    let mut root_variables = Vec::new();
    let mut reactive_declarations = Vec::new();

    for stmt in &script.body {
        match stmt {
            // finding all the root variables
            Stmt::Decl(d) => match d {
                Decl::Var(vd) => {
                    for v in &vd.decls {
                        match &v.name {
                            Pat::Ident(i) => root_variables.push(i.id.sym.to_string()),
                            _ => panic!("{:#?}", d),
                        }
                    }
                }
                _ => panic!("{:#?}", d),
            },

            // finding all the reactive declarations labeled with $:
            Stmt::Labeled(ls) => match &*ls.body {
                Stmt::Expr(expr) => match &*expr.expr {
                    // for now we assume that the reactive declaration is an assignment -> double = counter * 2
                    // this needs to be expanded to match Expr::Arrow (double = () => { counter * 2 })
                    // and other possible expressions and extract all the dependents from the right side
                    Expr::Assign(ae) => {
                        let mut left = match &ae.left {
                            PatOrExpr::Expr(e) => e.extract_names(),
                            PatOrExpr::Pat(p) => match &**p {
                                Pat::Ident(i) => vec![i.sym.to_string()],

                                Pat::Expr(e) => e.extract_names(),
                                _ => panic!("{:#?}", p),
                            },
                        };
                        let right = ae.right.extract_names();

                        let rd = ReactiveDeclaration {
                            assignees: left.clone(),
                            dependencies: right,
                            node: Expr::Assign(ae.clone()),
                        };

                        reactive_declarations.push(rd);
                        root_variables.append(&mut left);
                    }
                    _ => panic!("{:#?}", expr),
                },
                _ => panic!("{:#?}", ls),
            },
            _ => panic!("{:#?}", stmt),
        }
    }

    (root_variables, reactive_declarations)
}
