use std::{collections::HashMap, f32::consts::E};

use magic_string::SourceMap;
use strum_macros::Display;
use swc_estree_ast::{AssignmentExpression, Program};

use super::node::Node;

#[derive(Clone, Debug)]
pub struct BaseNode {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub node_type: String,
    pub children: Vec<TemplateNode>,
    pub prop_name: HashMap<String, TemplateNode>,
    pub expression: Option<Node>,
}

impl BaseNode {
    fn new(node_type: String) -> BaseNode {
        BaseNode {
            start: Some(0),
            end: Some(0),
            node_type,
            children: Vec::new(),
            prop_name: HashMap::new(),
            expression: None,
        }
    }
}

impl TmpNode for BaseNode {
    fn get_base_node(&mut self) -> &mut BaseNode {
        self
    }
}

#[derive(Clone, Debug)]
pub struct Fragment {
    pub base_node: BaseNode,
}

impl Fragment {
    pub fn new() -> Fragment {
        Fragment {
            base_node: BaseNode::new("Fragment".to_string()),
        }
    }
}

// This trait allows for different concrete types when matching a TemplateNode enum
pub trait TmpNode {
    fn get_base_node(&mut self) -> &mut BaseNode;
    fn get_children(&mut self) -> &mut Vec<TemplateNode> {
        &mut self.get_base_node().children
    }
    fn shift_children(&mut self) {
        self.get_base_node().children.remove(0);
    }
    fn pop_children(&mut self) {
        self.get_base_node().children.pop();
    }
}

#[derive(Clone, Debug)]
pub struct Text {
    pub base_node: BaseNode,
    pub data: String,
}

impl Text {
    pub fn new(data: String) -> Text {
        Text {
            base_node: BaseNode::new("Text".to_string()),
            data,
        }
    }
}

impl TmpNode for Text {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct MustacheTag {
    pub base_node: BaseNode,
    expression: Node,
}

impl MustacheTag {
    pub fn new(raw_mustache_tag: bool, expression: Node) -> Self {
        if raw_mustache_tag {
            MustacheTag {
                base_node: BaseNode::new("RawMustacheTag".to_string()),
                expression,
            }
        } else {
            MustacheTag {
                base_node: BaseNode::new("MustacheTag".to_string()),
                expression,
            }
        }
    }

    pub fn new_with_base_node(base_node: BaseNode, expression: Node) -> Self {
        Self {
            base_node,
            expression,
        }
    }
}

impl TmpNode for MustacheTag {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct Comment {
    pub base_node: BaseNode,
    pub data: String,
    pub ignores: Vec<String>,
}

impl Comment {
    pub fn new(data: String, ignores: Vec<String>) -> Comment {
        Comment {
            base_node: BaseNode::new("Comment".to_string()),
            data,
            ignores,
        }
    }
}

impl TmpNode for Comment {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct ConstTag {
    pub base_node: BaseNode,
    pub expression: AssignmentExpression,
}

impl ConstTag {
    pub fn new(expression: AssignmentExpression) -> ConstTag {
        ConstTag {
            base_node: BaseNode::new("ConstTag".to_string()),
            expression,
        }
    }
}

impl TmpNode for ConstTag {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct DebugTag {
    pub base_node: BaseNode,
    pub identifiers: Vec<Node>,
}

impl DebugTag {
    pub fn new(identifiers: Vec<Node>) -> DebugTag {
        DebugTag {
            base_node: BaseNode::new("DebugTag".to_string()),
            identifiers,
        }
    }
}

impl TmpNode for DebugTag {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Display)]
pub enum DirectiveType {
    Action,
    Animation,
    Binding,
    Class,
    StyleDirective,
    EventHandler,
    Let,
    Ref,
    Transition,
}

impl From<DirectiveType> for &'static str {
    fn from(value: DirectiveType) -> Self {
        match value {
            DirectiveType::Action => "Action",
            DirectiveType::Animation => "Animation",
            DirectiveType::Binding => "Binding",
            DirectiveType::Class => "Class",
            DirectiveType::StyleDirective => "StyleDirective",
            DirectiveType::EventHandler => "EventHandler",
            DirectiveType::Let => "Let",
            DirectiveType::Ref => "Ref",
            DirectiveType::Transition => "Transition",
        }
    }
}

#[derive(Clone, Debug)]
pub struct BaseDirective {
    pub base_node: BaseNode,
    pub name: String,
}

impl BaseDirective {
    pub fn new(directive_type: DirectiveType, name: String) -> BaseDirective {
        BaseDirective {
            base_node: BaseNode::new(directive_type.to_string()),
            name,
        }
    }
}

impl TmpNode for BaseDirective {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct BaseExpressionDirective {
    pub base_directive: BaseDirective,
    pub expression: Option<Node>,
    pub name: String,
    pub modifiers: Vec<String>,
}

impl BaseExpressionDirective {
    pub fn new(
        directive_type: DirectiveType,
        expression: Option<Node>,
        name: String,
        modifiers: Vec<String>,
    ) -> BaseExpressionDirective {
        BaseExpressionDirective {
            base_directive: BaseDirective::new(directive_type, name.clone()),
            expression,
            name,
            modifiers,
        }
    }
}

#[derive(Display)]
pub enum ElementType {
    InlineComponent,
    SlotTemplate,
    Title,
    Slot,
    Element,
    Head,
    Options,
    Window,
    Body,
}

impl From<ElementType> for &'static str {
    fn from(value: ElementType) -> Self {
        match value {
            ElementType::InlineComponent => "InlineComponent",
            ElementType::SlotTemplate => "SlotTemplate",
            ElementType::Title => "Title",
            ElementType::Slot => "Slot",
            ElementType::Element => "Element",
            ElementType::Head => "Head",
            ElementType::Options => "Options",
            ElementType::Window => "Window",
            ElementType::Body => "Body",
        }
    }
}

#[derive(Clone, Debug)]
pub enum ElementAttributes {
    BaseDirective(BaseDirective),
    Attribute(Attribute),
    SpreadAttribute(SpreadAttribute),
}

#[derive(Clone, Debug)]
pub struct Element {
    pub base_node: BaseNode,
    pub name: String,
    pub attributes: Vec<ElementAttributes>,
}

impl Element {
    pub fn new(
        element_type: ElementType,
        attributes: Vec<ElementAttributes>,
        name: String,
    ) -> Element {
        Element {
            base_node: BaseNode::new(element_type.to_string()),
            name,
            attributes,
        }
    }
}

impl TmpNode for Element {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub base_node: BaseNode,
    pub name: String,
    pub value: Vec<String>,
}

impl Attribute {
    pub fn new(name: String, value: Vec<String>) -> Attribute {
        Attribute {
            base_node: BaseNode::new("Attribute".to_string()),
            name,
            value,
        }
    }
}

impl TmpNode for Attribute {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct SpreadAttribute {
    pub base_node: BaseNode,
    pub expression: Node,
}

impl SpreadAttribute {
    pub fn new(expression: Node) -> SpreadAttribute {
        SpreadAttribute {
            base_node: BaseNode::new("Spread".to_string()),
            expression,
        }
    }
}

impl TmpNode for SpreadAttribute {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_node
    }
}

#[derive(Clone, Debug)]
pub struct Transition {
    pub base_expression_directive: BaseExpressionDirective,
    pub intro: bool,
    pub outro: bool,
}

impl Transition {
    pub fn new(intro: bool, outro: bool) -> Transition {
        Transition {
            base_expression_directive: BaseExpressionDirective::new(
                DirectiveType::Transition,
                None,
                String::new(),
                Vec::new(),
            ),
            intro,
            outro,
        }
    }
}

impl TmpNode for Transition {
    fn get_base_node(&mut self) -> &mut BaseNode {
        &mut self.base_expression_directive.base_directive.base_node
    }
}

#[derive(Clone, Debug)]
pub enum Directive {
    BaseDirective(BaseDirective),
    BaseExpressionDirective(BaseExpressionDirective),
    Transition(Transition),
}

impl TmpNode for Directive {
    fn get_base_node(&mut self) -> &mut BaseNode {
        match self {
            Directive::BaseDirective(bd) => bd.get_base_node(),
            Directive::BaseExpressionDirective(bed) => bed.base_directive.get_base_node(),
            Directive::Transition(t) => t.get_base_node(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TemplateNode {
    Text(Text),
    ConstTag(ConstTag),
    DebugTag(DebugTag),
    MustacheTag(MustacheTag),
    BaseNode(BaseNode),
    Element(Element),
    Attribute(Attribute),
    SpreadAttribute(SpreadAttribute),
    Directive(Directive),
    Transition(Transition),
    Comment(Comment),
}

impl TemplateNode {
    pub fn to_string(&self) -> String {
        match self.get_type().as_str() {
            "IfBlock" => "{#if} block".to_string(),
            "ThenBlock" => "{:then} block".to_string(),
            "ElseBlock" => "{:else} block".to_string(),
            "PendingBlock" | "AwaitBlock" => "{#await} block".to_string(),
            "CatchBlock" => "{:catch} block".to_string(),
            "EachBlock" => "{#each} block".to_string(),
            "RawMustacheTag" => "{@html} block".to_string(),
            "DebugTag" => "{@debug} block".to_string(),
            "ConstTag" => "{@const} tag".to_string(),
            "Element" | "InlineComponent" | "Slot" | "Title" => match self {
                TemplateNode::Element(e) => return format!("<{}> tag", e.name),
                _ => panic!("This shouldn't happen"),
            },
            default => String::from(default),
        }
    }

    pub fn get_type(&self) -> String {
        match self {
            TemplateNode::Text(t) => t.base_node.node_type.clone(),
            TemplateNode::ConstTag(c) => c.base_node.node_type.clone(),
            TemplateNode::DebugTag(d) => d.base_node.node_type.clone(),
            TemplateNode::MustacheTag(m) => m.base_node.node_type.clone(),
            TemplateNode::BaseNode(b) => b.node_type.clone(),
            TemplateNode::Element(e) => e.base_node.node_type.clone(),
            TemplateNode::Attribute(a) => a.base_node.node_type.clone(),
            TemplateNode::SpreadAttribute(s) => s.base_node.node_type.clone(),
            TemplateNode::Directive(d) => match d {
                Directive::BaseDirective(b) => b.base_node.node_type.clone(),
                Directive::BaseExpressionDirective(b) => {
                    b.base_directive.base_node.node_type.clone()
                }
                Directive::Transition(t) => t
                    .base_expression_directive
                    .base_directive
                    .base_node
                    .node_type
                    .clone(),
            },
            TemplateNode::Transition(t) => t
                .base_expression_directive
                .base_directive
                .base_node
                .node_type
                .clone(),
            TemplateNode::Comment(c) => c.base_node.node_type.clone(),
        }
    }

    pub fn get_data(&self) -> String {
        match self {
            TemplateNode::Text(el) => el.data.clone(),
            TemplateNode::Comment(el) => el.data.clone(),
            _ => panic!("unsupported this type"),
        }
    }

    pub fn get_prop(&self, name: &str) -> Option<TemplateNode> {
        match self {
            TemplateNode::Text(el) => el.base_node.prop_name.get(name).cloned(),
            TemplateNode::Comment(el) => el.base_node.prop_name.get(name).cloned(),
            TemplateNode::MustacheTag(el) => el.base_node.prop_name.get(name).cloned(),
            TemplateNode::ConstTag(el) => el.base_node.prop_name.get(name).cloned(),
            TemplateNode::DebugTag(el) => el.base_node.prop_name.get(name).cloned(),
            TemplateNode::Element(el) => el.base_node.prop_name.get(name).cloned(),
            TemplateNode::Attribute(el) => el.base_node.prop_name.get(name).cloned(),
            TemplateNode::SpreadAttribute(el) => el.base_node.prop_name.get(name).cloned(),
            _ => panic!("get_prop called on unsupported type"),
        }
    }

    pub fn unwrap(&mut self) -> &mut dyn TmpNode {
        match self {
            TemplateNode::Text(Text) => Text,
            TemplateNode::ConstTag(ConstTag) => ConstTag,
            TemplateNode::DebugTag(DebugTag) => DebugTag,
            TemplateNode::MustacheTag(MustacheTag) => MustacheTag,
            TemplateNode::BaseNode(BaseNode) => BaseNode,
            TemplateNode::Element(Element) => Element,
            TemplateNode::Attribute(Attribute) => Attribute,
            TemplateNode::SpreadAttribute(SpreadAttribute) => SpreadAttribute,
            TemplateNode::Directive(Directive) => Directive,
            TemplateNode::Transition(Transition) => Transition,
            TemplateNode::Comment(Comment) => Comment,
        }
    }

    pub fn get_children(&mut self) -> &mut Vec<TemplateNode> {
        match self {
            TemplateNode::Text(el) => el.get_children(),
            TemplateNode::ConstTag(el) => el.get_children(),
            TemplateNode::DebugTag(el) => el.get_children(),
            TemplateNode::MustacheTag(el) => el.get_children(),
            TemplateNode::BaseNode(el) => el.get_children(),
            TemplateNode::Element(el) => el.get_children(),
            TemplateNode::Attribute(el) => el.get_children(),
            TemplateNode::SpreadAttribute(el) => el.get_children(),
            TemplateNode::Directive(el) => el.get_children(),
            TemplateNode::Transition(el) => el.get_children(),
            TemplateNode::Comment(el) => el.get_children(),
        }
    }

    pub fn get_base_node(&mut self) -> &mut BaseNode {
        match self {
            TemplateNode::Text(e) => e.get_base_node(),
            TemplateNode::ConstTag(e) => e.get_base_node(),
            TemplateNode::DebugTag(e) => e.get_base_node(),
            TemplateNode::MustacheTag(e) => e.get_base_node(),
            TemplateNode::BaseNode(e) => e.get_base_node(),
            TemplateNode::Element(e) => e.get_base_node(),
            TemplateNode::Attribute(e) => e.get_base_node(),
            TemplateNode::SpreadAttribute(e) => e.get_base_node(),
            TemplateNode::Directive(e) => e.get_base_node(),
            TemplateNode::Transition(e) => e.get_base_node(),
            TemplateNode::Comment(e) => e.get_base_node(),
        }
    }

    pub fn get_name(&self) -> Option<String> {
        match self {
            TemplateNode::Element(e) => Some(e.name.clone()),
            TemplateNode::Attribute(a) => Some(a.name.clone()),
            TemplateNode::Directive(d) => match d {
                Directive::BaseDirective(b) => Some(b.name.clone()),
                Directive::BaseExpressionDirective(b) => Some(b.name.clone()),
                Directive::Transition(t) => None,
            },
            _ => None,
        }
    }
}
// We don't have interfaces in Rust
// So I guess we don't need this here?
// pub struct Parser {
//     pub template: String,
//     pub filename: Option<String>,
//     pub index: i32,
//     pub stack: Vec<Node>,
//     pub html: Node,
//     pub css: Node,
//     pub js: Node,
//     pub meta_tags: Vec<String>,
// }

#[derive(Clone, Debug)]
pub struct Script {
    pub base_node: BaseNode,
    pub context: String,
    pub content: Program,
}

impl Script {
    pub fn new(context: String, content: Program) -> Script {
        Script {
            base_node: BaseNode::new("Script".to_string()),
            context,
            content,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Style {
    pub base_node: BaseNode,
    // pub attributes: Vec<String>, // TODO - from svelte
    // pub children: Vec<String>,   // TODO add CSS node types - from svelte
    pub content: StyleContent,
}

impl Style {
    pub fn new(content: StyleContent) -> Style {
        Style {
            base_node: BaseNode::new("style".to_string()),
            content,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StyleContent {
    pub start: i32,
    pub end: i32,
    pub styles: String,
}

impl StyleContent {
    pub fn new(start: i32, end: i32, styles: String) -> StyleContent {
        StyleContent { start, end, styles }
    }
}

pub struct Ast {
    pub html: Fragment,
    pub css: Option<Style>,
    pub instance: Option<Script>,
    pub module: Option<Script>,
}

impl Ast {
    pub fn new(
        html: Fragment,
        css: Option<Style>,
        instance: Option<Script>,
        module: Option<Script>,
    ) -> Ast {
        Ast {
            html,
            css,
            instance,
            module,
        }
    }
}

pub struct WarningStart {
    pub line: i32,
    pub column: i32,
    pub pos: Option<i32>,
}

impl WarningStart {
    pub fn new(line: i32, column: i32, pos: Option<i32>) -> WarningStart {
        WarningStart { line, column, pos }
    }
}

pub struct WarningEnd {
    pub line: i32,
    pub column: i32,
}

impl WarningEnd {
    pub fn new(line: i32, column: i32) -> WarningEnd {
        WarningEnd { line, column }
    }
}
pub struct Warnning {
    pub start: Option<WarningStart>,
    pub end: Option<WarningEnd>,
    pub pos: Option<i32>,
    pub code: String,
    pub message: String,
    pub filename: Option<String>,
    pub frame: Option<String>,
}

impl Warnning {
    pub fn new(
        start: Option<WarningStart>,
        end: Option<WarningEnd>,
        pos: Option<i32>,
        code: String,
        message: String,
        filename: Option<String>,
        frame: Option<String>,
    ) -> Warnning {
        Warnning {
            start,
            end,
            pos,
            code,
            message,
            filename,
            frame,
        }
    }
}

pub enum ModuleFormat {
    Esm,
    Cjs,
}

pub enum EnableSourcemap {
    Enable(bool),
    Type { js: bool, css: bool },
}

pub struct CssHashGetter {
    name: String,
    filename: Option<String>,
    css: String,
    hash: String,
}

pub enum Generate {
    Dom,
    Ssr,
    Enable(bool),
}

pub enum ErrorMode {
    Throw,
    Warn,
}

pub enum VariablesReport {
    Full,
    Strict,
    Enable(bool),
}

pub struct CompileOptions {
    pub format: Option<ModuleFormat>,
    pub name: Option<String>,
    pub filename: Option<String>,
    pub generate: Option<Generate>,
    pub error_mode: Option<ErrorMode>,
    pub vars_report: Option<VariablesReport>,
    pub sourcemap: Option<String>, // object | string in svelte
    pub enable_sourcemap: Option<EnableSourcemap>,
    pub output_filename: Option<String>,
    pub css_output_filename: Option<String>,
    pub svelte_path: Option<String>,
    pub dev: Option<bool>,
    pub accesors: Option<bool>,
    pub immutable: Option<bool>,
    pub hydratable: Option<bool>,
    pub legacy: Option<bool>,
    pub custom_element: Option<bool>,
    pub tag: Option<String>,
    pub css: Option<bool>,
    pub loop_guard_timenout: Option<i32>,
    pub namespace: Option<String>,
    pub css_hash: Option<CssHashGetter>,
    pub preserve_comments: Option<bool>,
    pub preserve_whitespace: Option<bool>,
}

pub struct ParserOptions {
    pub filename: Option<String>,
    pub custom_element: bool,
}

pub struct Visitor {
    // **Typescript**
    // enter: (node: Node) => void;
    // leave?: (node: Node) => void;
}

pub struct AppendTarget {
    pub slots: HashMap<String, String>,
    pub slot_stack: Vec<String>,
}

pub struct Var {
    pub name: String,
    pub export_name: Option<String>, // the `bar` in `export { foo as bar }`
    pub injected: Option<bool>,
    pub module: Option<bool>,
    pub mutated: Option<bool>,
    pub reassigned: Option<bool>,
    pub referenced: Option<bool>, // referenced from template scope
    pub referenced_from_script: Option<bool>, // referenced from script

    // used internally, but not exposed
    pub global: Option<bool>,
    pub internal: Option<bool>, // event handlers, bindings
    pub initialised: Option<bool>,
    pub hoistable: Option<bool>,
    pub subscribable: Option<bool>,
    pub is_reactive_dependency: Option<bool>,
    pub imported: Option<bool>,
}

pub struct CssResult {
    pub code: String,
    pub map: SourceMap,
}

impl CssResult {
    pub fn new(code: String, map: SourceMap) -> CssResult {
        CssResult { code, map }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_simple() {
        assert_eq!(1, 1)
    }
}
