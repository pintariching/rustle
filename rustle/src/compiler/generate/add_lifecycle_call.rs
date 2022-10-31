use std::collections::HashSet;

use swc_common::Span;
use swc_ecma_ast::{
    ArrayLit, BlockStmtOrExpr, CallExpr, Callee, Decl, Expr, ExprOrSpread, ExprStmt, Ident, Lit,
    ParenExpr, Script, SeqExpr, Stmt, Str,
};

use crate::compiler::expr_visitor::Visit;

/// Adds the calls to the `lifecycle.update()` function
/// For example:
///
/// ```ignore
/// // javascript
/// const increment () => {
///     counter++;
/// }
/// ```
///
/// turns into
/// ```ignore
/// // javascript
/// const increment () => {
///     counter++;
///     update(["counter"]);
/// }
/// ```
///
/// and
///
/// ```ignore
/// // javascript
/// const increment () => counter++;
/// ```
/// turns into
/// ```ignore
/// // javascript
/// const increment () => (counter++, update(["counter"]));
/// ```
pub fn add_lifecycle_calls(mut script: Script, will_be_updated: &HashSet<String>) -> Script {
    for stmt in &mut script.body {
        match stmt {
            Stmt::Decl(decl) => match decl {
                Decl::Var(vd) => {
                    for v in &mut vd.decls {
                        if let Some(expr) = &mut v.init {
                            update_body_ast(expr, &will_be_updated);
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    script
}

// Updates an expression and adds the `lifecycle.update()` call
fn update_body_ast(expr: &mut Expr, will_be_updated: &HashSet<String>) {
    match expr {
        Expr::Arrow(ae) => match &mut ae.body {
            BlockStmtOrExpr::BlockStmt(bs) => {
                let mut names_to_update = Vec::new();

                // check all variable names in a block statement if they're in the will_be_updated list
                for stmt in &mut bs.stmts {
                    match stmt {
                        Stmt::Expr(expr) => {
                            for name in expr.expr.extract_names() {
                                if will_be_updated.contains(&name) {
                                    names_to_update.push(name);
                                }
                            }
                        }
                        _ => (),
                    }
                }

                // for every variable that will be changed, insert a `update([name]);`
                // statement at the end of the block statement

                // const increment () => {
                //     update(["counter"]);
                // }
                //          |
                //          V
                // const increment () => {
                //     counter++;
                //     update(["counter"]);
                // }
                for name in names_to_update {
                    bs.stmts.push(Stmt::Expr(ExprStmt {
                        span: Span::default(),
                        expr: Box::new(Expr::Call(lifecycle_update_ast(&name))),
                    }))
                }
            }
            BlockStmtOrExpr::Expr(expr) => {
                let mut names_to_update = Vec::new();

                for name in expr.extract_names() {
                    if will_be_updated.contains(&name) {
                        names_to_update.push(name)
                    }
                }

                let mut new_exprs = vec![expr.clone()];

                // const increment () => counter++;
                //                              |
                //                              V
                // const increment () => (counter++, update(["counter"]));

                for name in names_to_update {
                    new_exprs.push(Box::new(Expr::Call(lifecycle_update_ast(&name))));
                }

                let paren_body = ParenExpr {
                    span: Span::default(),
                    expr: Box::new(Expr::Seq(SeqExpr {
                        span: Span::default(),
                        exprs: new_exprs,
                    })),
                };

                ae.body = BlockStmtOrExpr::Expr(Box::new(Expr::Paren(paren_body)));
            }
        },
        _ => (),
    }
}

/// Returns a call expression to the update function `update([variable_name])`
/// as an `swc_ecma_ast::CallExpr`
fn lifecycle_update_ast(variable_name: &str) -> CallExpr {
    CallExpr {
        span: Span::default(),
        callee: Callee::Expr(Box::new(Expr::Ident(Ident {
            span: Span::default(),
            sym: "update".into(),
            optional: false,
        }))),
        args: vec![ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Array(ArrayLit {
                span: Span::default(),
                elems: vec![Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: Span::default(),
                        value: variable_name.trim().into(),
                        raw: Some(format!("\"{}\"", variable_name).into()),
                    }))),
                })],
            })),
        }],
        type_args: None,
    }
}
