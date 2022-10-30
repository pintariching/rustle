use swc_ecma_ast::{Decl, Expr, Pat, PatOrExpr, Script, Stmt};

use crate::compiler::expr_visitor::Visit;

use super::ReactiveDeclaration;

pub fn extract_root_variables(script: &Script) -> (Vec<String>, Vec<ReactiveDeclaration>) {
    let mut root_variables = Vec::new();
    let mut reactive_declarations = Vec::new();

    for stmt in &script.body {
        match stmt {
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
            Stmt::Labeled(ls) => match &*ls.body {
                Stmt::Decl(d) => match d {
                    Decl::Var(vd) => {
                        for v in &vd.decls {
                            root_variables.append(&mut v.init.clone().unwrap().extract_names());
                        }
                    }
                    _ => panic!("{:#?}", d),
                },
                Stmt::Expr(expr) => match &*expr.expr {
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
