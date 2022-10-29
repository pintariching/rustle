use swc_ecma_ast::{ArrowExpr, Decl, Expr, Stmt, VarDecl, VarDeclarator};

pub trait VisitArrowExpr {
    fn visit_stmt<'a>(&'a mut self, s: &'a mut Stmt) -> Option<&mut ArrowExpr>;
    fn visit_decl<'a>(&'a mut self, d: &'a mut Decl) -> Option<&mut ArrowExpr>;
    fn visit_var_decl<'a>(&'a mut self, vd: &'a mut VarDecl) -> Option<&mut ArrowExpr>;
    fn visit_var_declarator<'a>(&'a mut self, vd: &'a mut VarDeclarator) -> Option<&mut ArrowExpr>;
    fn visit_expr<'a>(&'a mut self, e: &'a mut Expr) -> Option<&mut ArrowExpr>;
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
