use swc_ecma_ast::{Decl, Script, Stmt};

use crate::compiler::expr_visitor::Visit;

/// Extracts variables that will change in function calls or other expressions.
///
/// For example: `counter` and `double` will be changed if `increment` is called
/// ```ignore
/// // javascript
/// let counter = 1;
/// const increment = () => counter++;
/// $: double = counter * 2;
/// ```
pub fn extract_variables_that_change(script: &Script) -> Vec<String> {
    let mut changed_vars = Vec::new();

    for stmt in &script.body {
        match stmt {
            Stmt::Decl(decl) => match decl {
                Decl::Var(vdcl) => {
                    // a variable declaration can declare multiple variables
                    // let [name, age, message] = ["Nathan", 28, "Hello there!"];
                    for decl in &vdcl.decls {
                        if let Some(expr) = &decl.init {
                            changed_vars.append(&mut expr.extract_updated_names())
                        }
                    }
                }
                _ => (),
            },
            Stmt::Labeled(ls) => match &*ls.body {
                // for now we assume that the reactive declaration is an assignment -> double = counter * 2
                // this needs to be expanded to match Expr::Arrow (double = () => { counter * 2 })
                // and other possible expressions and extract variables that will change
                Stmt::Expr(expr) => changed_vars.append(&mut expr.expr.extract_updated_names()),
                _ => (),
            },
            _ => (),
        }
    }

    changed_vars
}
