use swc_ecma_ast::{Decl, Script, Stmt};

use crate::compiler::expr_visitor::Visit;

pub fn extract_variables_that_change(script: &Script) -> Vec<String> {
    let mut changed_vars = Vec::new();

    for stmt in &script.body {
        match stmt {
            Stmt::Expr(_) => todo!(),
            Stmt::Block(_) => todo!(),
            Stmt::Decl(decl) => match decl {
                Decl::Var(vdcl) => {
                    for decl in &vdcl.decls {
                        if let Some(expr) = &decl.init {
                            changed_vars.append(&mut expr.extract_updated_names())
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    changed_vars
}
