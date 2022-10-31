use serde::Serialize;
use swc_css_ast::Stylesheet;
use swc_ecma_ast::{Expr, Script};

#[derive(Debug, Serialize)]
pub struct RustleAst {
    pub script: Option<Script>,
    pub style: Option<Stylesheet>,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Serialize)]
pub struct RustleAttribute {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, Serialize)]
pub enum AttributeValue {
    Expr(Expr),
    String(String),
}

#[derive(Debug, Serialize)]
pub struct RustleElement {
    pub name: String,
    pub attributes: Vec<RustleAttribute>,
    pub fragments: Vec<Fragment>,
}

#[derive(Debug, Serialize)]
pub struct RustleText {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub enum Fragment {
    Script(Script),
    Style(Stylesheet),
    Element(RustleElement),
    Expression(Expr),
    Text(RustleText),
}
