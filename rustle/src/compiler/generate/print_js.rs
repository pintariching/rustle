use swc_common::{sync::Lrc, SourceMap, Span};
use swc_ecma_ast::{EsVersion, Expr, ExprStmt, Script, Stmt};
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};

pub fn generate_js_from_script(script: Script) -> String {
    let mut buffer = Vec::new();
    {
        let cm: Lrc<SourceMap> = Default::default();
        let writer = JsWriter::new(cm.clone(), "\n", &mut buffer, None);
        let config = Config {
            target: EsVersion::latest(),
            ascii_only: false,
            minify: false,
            omit_last_semi: false,
        };
        let mut emmiter = Emitter {
            cfg: config,
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };
        emmiter.emit_script(&script).unwrap();
    }

    String::from_utf8(buffer).unwrap()
}

pub fn generate_js_from_expr(expr: &Expr) -> String {
    generate_js_from_script(script_from_expr(expr))
}

pub fn script_from_expr(expr: &Expr) -> Script {
    Script {
        span: Span::default(),
        body: vec![Stmt::Expr(ExprStmt {
            span: Span::default(),
            expr: Box::new(expr.clone()),
        })],
        shebang: None,
    }
}
