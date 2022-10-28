use swc_common::Span;
use swc_ecma_ast::{
    ArrayLit, ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, Ident,
    Lit, MemberExpr, MemberProp, ParenExpr, PatOrExpr, Script, SeqExpr, Stmt, Str,
};

use crate::compiler::swc_ast_visitor::{ArrowInterpreter, VisitArrowExpr};

pub fn add_lifecycle_calls(mut script: Script) -> Script {
    let mut interpreter = ArrowInterpreter;

    let mut stmts = script.body.clone();

    for stmt in &mut stmts {
        // match stmt {
        //     Stmt::Expr(expr_stmt) => match expr_stmt.expr.unwrap_parens() {
        //         Expr::Update(ue) => todo!(),
        //         Expr::Assign(ae) => todo!(),
        //         _ => todo!(),
        //     },
        //     _ => todo!(),
        // }

        if let Some(mut arrow) = ArrowInterpreter::visit_stmt(&mut interpreter, stmt) {
            update_body_ast(&mut arrow);
        }
    }

    script.body = stmts;
    script
}

fn update_body_ast(arrow_expr: &mut ArrowExpr) {
    match arrow_expr.body.clone() {
        BlockStmtOrExpr::BlockStmt(bs) => {
            let mut names = Vec::new();
            for stmt in bs.stmts {
                match stmt {
                    Stmt::Expr(expr) => match expr.expr.unwrap_parens() {
                        Expr::Assign(a) => match &a.left {
                            PatOrExpr::Expr(e) => match e.unwrap_parens() {
                                Expr::Ident(i) => names.push(i.sym.to_string()),
                                _ => (),
                            },
                            PatOrExpr::Pat(_) => (),
                        },
                        _ => (),
                    },
                    _ => (),
                };
            }

            for name in names {
                arrow_expr
                    .body
                    .as_mut_block_stmt()
                    .unwrap()
                    .stmts
                    .push(Stmt::Expr(ExprStmt {
                        span: Span::default(),
                        expr: Box::new(Expr::Call(lifecycle_update_ast(&name))),
                    }));
            }
        }
        BlockStmtOrExpr::Expr(expr) => match expr.unwrap_parens() {
            Expr::Update(ue) => {
                let variable_name = ue.arg.clone().ident().unwrap().sym.to_string();

                let new_body = ParenExpr {
                    span: Span::default(),
                    expr: Box::new(Expr::Seq(SeqExpr {
                        span: Span::default(),
                        exprs: vec![
                            Box::new(Expr::Update(ue.clone())),
                            Box::new(Expr::Call(lifecycle_update_ast(&variable_name))),
                        ],
                    })),
                };

                arrow_expr.body = BlockStmtOrExpr::Expr(Box::new(Expr::Paren(new_body)));
            }
            _ => (),
        },
    }
}

fn lifecycle_update_ast(variable_name: &str) -> CallExpr {
    CallExpr {
        span: Span::default(),
        callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
            span: Span::default(),
            obj: Box::new(Expr::Ident(Ident {
                span: Span::default(),
                sym: "lifecycle".into(),
                optional: false,
            })),
            prop: MemberProp::Ident(Ident {
                span: Span::default(),
                sym: "update".into(),
                optional: false,
            }),
        }))),
        args: vec![ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Array(ArrayLit {
                span: Span::default(),
                elems: vec![Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: Span::default(),
                        value: variable_name.into(),
                        raw: Some(format!("\"{}\"", variable_name).into()),
                    }))),
                })],
            })),
        }],
        type_args: None,
    }
}
