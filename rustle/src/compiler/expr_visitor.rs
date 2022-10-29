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

fn recursive_extract(expr: &Expr, buf: &mut Vec<String>) {
    match expr {
        Expr::Ident(i) => buf.push(i.sym.to_string()),
        Expr::Bin(be) => {
            let left = &*be.left;
            let right = &*be.right;

            recursive_extract(left, buf);
            recursive_extract(right, buf);
        }
        Expr::Call(ce) => {
            for arg in &ce.args {
                recursive_extract(&arg.expr, buf);
            }
        }
        Expr::Member(me) => recursive_extract(&me.obj, buf),
        Expr::Update(ue) => recursive_extract(&*ue.arg, buf),
        _ => println!("Unsupported expression: {:#?}", expr),
    }
}

fn single_recursive_extract(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Ident(i) => Some(i.sym.to_string()),
        _ => None,
    }
}

fn recursive_updated_extract(expr: &Expr, buf: &mut Vec<String>) {
    match expr {
        Expr::Update(ue) => buf.push(single_recursive_extract(&*ue.arg).unwrap()),
        Expr::Arrow(ae) => match &ae.body {
            BlockStmtOrExpr::Expr(e) => recursive_updated_extract(&*e, buf),
            BlockStmtOrExpr::BlockStmt(bs) => {
                for stmt in &bs.stmts {
                    match stmt {
                        Stmt::Expr(es) => recursive_updated_extract(&*es.expr, buf),
                        _ => println!("Unsupported statement: {:#?}", stmt),
                    }
                }
            }
        },
        Expr::Assign(ae) => match &ae.left {
            PatOrExpr::Expr(expr) => buf.push(single_recursive_extract(&*expr).unwrap()),
            PatOrExpr::Pat(pe) => match &**pe {
                Pat::Ident(i) => buf.push(i.sym.to_string()),
                _ => println!("Unsupported pattern or expression: {:#?}", pe),
            },
        },
        Expr::Member(me) => buf.push(single_recursive_extract(&*me.obj).unwrap()),
        Expr::Lit(_) => (),
        _ => println!("Unsupported expression: {:#?}", expr),
    }
}
