use swc_ecma_ast::Expr;

pub trait Visit {
    fn extract_names(&self) -> Vec<String>;
}

impl Visit for Expr {
    fn extract_names(&self) -> Vec<String> {
        let mut buf = Vec::new();
        recursive_extract(self, &mut buf);

        buf
    }
}

fn recursive_extract(expr: &Expr, buf: &mut Vec<String>) {
    match expr {
        Expr::Ident(i) => buf.push(i.sym.to_string()),
        Expr::Bin(be) => {
            let left = be.left.unwrap_parens();
            let right = be.right.unwrap_parens();

            recursive_extract(left, buf);
            recursive_extract(right, buf);
        }
        Expr::Call(ce) => {
            // TODO: Handle this better
            let n = ce
                .args
                .first()
                .unwrap()
                .expr
                .clone()
                .ident()
                .unwrap()
                .sym
                .to_string();

            buf.push(n);
        }
        _ => println!("Unsupported expression: {:#?}", expr),
    }
}
