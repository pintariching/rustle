use swc_ecma_ast::{
    ArrowExpr, Decl, Expr, Ident, Script, Stmt, UpdateExpr, VarDecl, VarDeclarator,
};

pub fn extract_variables_that_change(script: &Script) -> Vec<String> {
    let body = script.body.clone();

    let mut interpreter = Interpreter;
    let vars = body
        .iter()
        .filter_map(|stmt| Interpreter::visit_stmt(&mut interpreter, stmt))
        .collect::<Vec<String>>();

    vars
}

trait IdentVisitor<T> {
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_decl(&mut self, d: &Decl) -> T;
    fn visit_var_decl(&mut self, vd: &VarDecl) -> T;
    fn visit_var_declarator(&mut self, vd: &VarDeclarator) -> T;
    fn visit_expr(&mut self, e: &Expr) -> T;
    fn visit_arrow_expr(&mut self, ae: &ArrowExpr) -> T;
    fn visit_update_expr(&mut self, ue: &UpdateExpr) -> T;
    fn visit_identifier(&mut self, i: &Ident) -> T;
}

struct Interpreter;
impl IdentVisitor<Option<String>> for Interpreter {
    fn visit_stmt(&mut self, s: &Stmt) -> Option<String> {
        match s {
            Stmt::Decl(d) => self.visit_decl(d),
            _ => None,
        }
    }

    fn visit_decl(&mut self, d: &Decl) -> Option<String> {
        match d {
            Decl::Var(vd) => self.visit_var_decl(vd),
            _ => None,
        }
    }

    fn visit_var_decl(&mut self, vd: &VarDecl) -> Option<String> {
        match vd.decls.first() {
            Some(vd) => self.visit_var_declarator(vd),
            None => None,
        }
    }

    fn visit_var_declarator(&mut self, vd: &VarDeclarator) -> Option<String> {
        match &vd.init {
            Some(e) => self.visit_expr(e),
            None => None,
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> Option<String> {
        match e {
            Expr::Update(ue) => self.visit_update_expr(ue),
            Expr::Arrow(ae) => self.visit_arrow_expr(ae),
            _ => None,
        }
    }

    fn visit_arrow_expr(&mut self, ae: &ArrowExpr) -> Option<String> {
        match &ae.body {
            swc_ecma_ast::BlockStmtOrExpr::BlockStmt(_) => None,
            swc_ecma_ast::BlockStmtOrExpr::Expr(e) => self.visit_expr(e),
        }
    }

    fn visit_update_expr(&mut self, ue: &UpdateExpr) -> Option<String> {
        match &ue.arg.unwrap_parens() {
            Expr::Ident(i) => self.visit_identifier(i),
            _ => None,
        }
    }

    fn visit_identifier(&mut self, i: &Ident) -> Option<String> {
        Some(i.sym.to_string())
    }
}
