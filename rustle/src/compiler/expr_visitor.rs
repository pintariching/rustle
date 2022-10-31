use core::panic;

use swc_ecma_ast::{BlockStmtOrExpr, Expr, Pat, PatOrExpr, Stmt};

pub trait Visit {
    fn extract_names(&self) -> Vec<String>;
    fn extract_first_name(&self) -> Option<String>;
    fn extract_updated_names(&self) -> Vec<String>;
}

impl Visit for Expr {
    fn extract_names(&self) -> Vec<String> {
        let mut buf = Vec::new();
        recursive_extract(self, &mut buf);

        buf
    }

    fn extract_first_name(&self) -> Option<String> {
        single_recursive_extract(self)
    }

    fn extract_updated_names(&self) -> Vec<String> {
        let mut buf = Vec::new();
        recursive_updated_extract(self, &mut buf);

        buf
    }
}

/// Extracts all the variable names from an expression
fn recursive_extract(expr: &Expr, buf: &mut Vec<String>) {
    match expr {
        Expr::Ident(i) => buf.push(i.sym.to_string()),
        Expr::Bin(be) => {
            recursive_extract(&*be.left, buf);
            recursive_extract(&*be.right, buf);
        }
        Expr::Call(ce) => {
            for arg in &ce.args {
                recursive_extract(&arg.expr, buf);
            }
        }
        Expr::Member(me) => recursive_extract(&me.obj, buf),
        Expr::Update(ue) => recursive_extract(&*ue.arg, buf),
        Expr::Assign(ae) => {
            extract_pat_or_expr(&ae.left, buf);
            recursive_extract(&ae.right, buf);
        }
        Expr::Lit(_) => (),
        Expr::Arrow(ae) => match &ae.body {
            BlockStmtOrExpr::BlockStmt(bs) => {
                for stmt in &bs.stmts {
                    match stmt {
                        Stmt::Expr(expr) => recursive_extract(&expr.expr, buf),
                        _ => panic!("{:#?}", stmt),
                    }
                }
            }
            BlockStmtOrExpr::Expr(expr) => recursive_extract(&expr, buf),
        },
        _ => panic!("{:#?}", expr),
    }
}

fn extract_pat_or_expr(pat_or_expr: &PatOrExpr, buf: &mut Vec<String>) {
    match pat_or_expr {
        PatOrExpr::Expr(expr) => recursive_extract(expr, buf),
        PatOrExpr::Pat(pat) => match &**pat {
            Pat::Ident(bi) => buf.push(bi.sym.to_string()),
            _ => panic!("{:#?}", pat),
        },
    }
}

fn single_recursive_extract(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Ident(i) => Some(i.sym.to_string()),
        // could be a deeply nested value `counter.count.value`
        Expr::Member(me) => single_recursive_extract(&me.obj),
        _ => panic!("{:#?}", expr),
    }
}

/// Recursively calls itself and extracts all variables that will change.
/// This is done by checking if the expression is an assignment -> `val = 5`
/// or an update expression -> `val++`.
fn recursive_updated_extract(expr: &Expr, buf: &mut Vec<String>) {
    match expr {
        Expr::Update(ue) => buf.push(single_recursive_extract(&*ue.arg).unwrap()),
        // an Expr::Arrow can be a simple expr () => counter++
        // or a block statement with multiple expressions () => { counter++; value++; }
        Expr::Arrow(ae) => match &ae.body {
            BlockStmtOrExpr::Expr(e) => recursive_updated_extract(&*e, buf),
            BlockStmtOrExpr::BlockStmt(bs) => {
                for stmt in &bs.stmts {
                    match stmt {
                        Stmt::Expr(es) => recursive_updated_extract(&*es.expr, buf),
                        _ => panic!("{:#?}", expr),
                    }
                }
            }
        },
        Expr::Assign(ae) => match &ae.left {
            PatOrExpr::Expr(expr) => buf.push(single_recursive_extract(&*expr).unwrap()),
            PatOrExpr::Pat(pe) => match &**pe {
                Pat::Ident(i) => buf.push(i.sym.to_string()),
                _ => panic!("{:#?}", pe),
            },
        },
        Expr::Member(me) => buf.push(single_recursive_extract(&*me.obj).unwrap()),
        // ignore Expr::Lit and Expr::Object as they don't update any values
        Expr::Lit(_) => (),
        Expr::Object(_) => (),
        _ => panic!("{:#?}", expr),
    }
}
