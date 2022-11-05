use serde::Serialize;
use swc_css_ast::Stylesheet;
use swc_ecma_ast::{Expr, ModuleItem, Program, Script};

#[derive(Debug, Serialize)]
pub struct RustleAst {
    pub script: Option<Script>,
    pub imports_exports: Option<Vec<ModuleItem>>,
    pub style: Option<Stylesheet>,
    pub fragments: Vec<Fragment>,
    pub is_component: bool,
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
    pub nested_component: bool,
}

#[derive(Debug, Serialize)]
pub struct RustleText {
    pub data: String,
}

#[derive(Debug, Serialize)]
pub enum Fragment {
    Program(Program),
    Style(Stylesheet),
    Element(RustleElement),
    NestedComponent(RustleElement),
    Expression(Expr),
    Text(RustleText),
}
