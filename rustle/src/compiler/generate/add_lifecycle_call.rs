use swc_common::Span;
use swc_ecma_ast::{
    ArrayLit, ArrowExpr, BlockStmtOrExpr, CallExpr, Callee, Expr, ExprOrSpread, Ident, Lit,
    MemberExpr, MemberProp, ParenExpr, Script, SeqExpr, Str,
};

use crate::compiler::swc_ast_visitor::{ArrowInterpreter, VisitArrowExpr};

pub fn add_lifecycle_calls(mut script: Script) -> Script {
    let mut interpreter = ArrowInterpreter;

    let mut stmts = script.body.clone();

    for stmt in &mut stmts {
        if let Some(mut arrow) = ArrowInterpreter::visit_stmt(&mut interpreter, stmt) {
            update_body_ast(&mut arrow);
        }
    }

    script.body = stmts;
    script
}

fn update_body_ast(arrow_expr: &mut ArrowExpr) {
    let body = arrow_expr.body.as_expr().unwrap().clone();
    let variable_name = body
        .as_update()
        .unwrap()
        .arg
        .clone()
        .ident()
        .unwrap()
        .sym
        .to_string();

    let new_body = ParenExpr {
        span: Span::default(),
        expr: Box::new(Expr::Seq(SeqExpr {
            span: Span::default(),
            exprs: vec![
                body,
                Box::new(Expr::Call(lifecycle_update_ast(&variable_name))),
            ],
        })),
    };

    arrow_expr.body = BlockStmtOrExpr::Expr(Box::new(Expr::Paren(new_body)));
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
