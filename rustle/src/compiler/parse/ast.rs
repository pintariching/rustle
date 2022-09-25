use serde::Serialize;
use swc_ecma_ast::{Expr, Script};
use swc_html_ast::Text;

#[derive(Serialize)]
pub struct RustleAst {
    pub script: Script,
    pub fragments: Vec<Fragment>,
}

#[derive(Serialize)]
pub struct RustleAttribute {
    pub name: String,
    pub value: Expr,
}

#[derive(Serialize)]
pub struct RustleElement {
    pub name: String,
    pub attributes: Vec<RustleAttribute>,
    pub fragments: Vec<Fragment>,
}

#[derive(Serialize)]
pub enum Fragment {
    Script(Script),
    Element(RustleElement),
    Expression(Expr),
    Text(Text),
}
