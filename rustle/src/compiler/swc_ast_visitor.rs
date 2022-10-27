use swc_ecma_ast::{ArrowExpr, Decl, Expr, Ident, Stmt, UpdateExpr, VarDecl, VarDeclarator};

pub trait Visitor<T> {
    fn visit_stmt(&mut self, s: &Stmt) -> T;
    fn visit_decl(&mut self, d: &Decl) -> T;
    fn visit_var_decl(&mut self, vd: &VarDecl) -> T;
    fn visit_var_declarator(&mut self, vd: &VarDeclarator) -> T;
    fn visit_expr(&mut self, e: &Expr) -> T;
    fn visit_arrow_expr(&mut self, ae: &ArrowExpr) -> T;
    fn visit_update_expr(&mut self, ue: &UpdateExpr) -> T;
    fn visit_identifier(&mut self, i: &Ident) -> T;
}

pub trait VisitArrowExpr {
    fn visit_stmt<'a>(&'a mut self, s: &'a mut Stmt) -> Option<&mut ArrowExpr>;
    fn visit_decl<'a>(&'a mut self, d: &'a mut Decl) -> Option<&mut ArrowExpr>;
    fn visit_var_decl<'a>(&'a mut self, vd: &'a mut VarDecl) -> Option<&mut ArrowExpr>;
    fn visit_var_declarator<'a>(&'a mut self, vd: &'a mut VarDeclarator) -> Option<&mut ArrowExpr>;
    fn visit_expr<'a>(&'a mut self, e: &'a mut Expr) -> Option<&mut ArrowExpr>;
}

pub struct Interpreter;
impl Visitor<Option<String>> for Interpreter {
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
            Expr::Ident(i) => self.visit_identifier(i),
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

pub struct ArrowInterpreter;
impl VisitArrowExpr for ArrowInterpreter {
    fn visit_stmt<'a>(&'a mut self, s: &'a mut Stmt) -> Option<&mut ArrowExpr> {
        match s {
            Stmt::Decl(d) => self.visit_decl(d),
            _ => None,
        }
    }

    fn visit_decl<'a>(&'a mut self, d: &'a mut Decl) -> Option<&mut ArrowExpr> {
        match d {
            Decl::Var(vd) => self.visit_var_decl(vd),
            _ => None,
        }
    }

    fn visit_var_decl<'a>(&'a mut self, vd: &'a mut VarDecl) -> Option<&mut ArrowExpr> {
        match vd.decls.iter_mut().nth(0) {
            Some(vd) => self.visit_var_declarator(vd),
            None => None,
        }
    }

    fn visit_var_declarator<'a>(&'a mut self, vd: &'a mut VarDeclarator) -> Option<&mut ArrowExpr> {
        match &mut vd.init {
            Some(e) => self.visit_expr(e),
            None => None,
        }
    }

    fn visit_expr<'a>(&'a mut self, e: &'a mut Expr) -> Option<&mut ArrowExpr> {
        match e {
            Expr::Arrow(ae) => Some(ae),
            _ => None,
        }
    }
}
