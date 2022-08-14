use std::collections::HashMap;

use swc_estree_ast::{AssignmentExpression, Program};

struct BaseNode {
    start: i32,
    end: i32,
    node_type: String,
    children: Option<Vec<TemplateNode>>,
    prop_name: Vec<String>,
}

pub struct Fragment {
    base_node: BaseNode,
}

pub struct Text {
    base_node: BaseNode,
    data: String,
}

pub struct MustacheTag {
    base_node: BaseNode,
    // expression: Node
}

pub struct Comment {
    base_node: BaseNode,
    data: String,
    ignores: Vec<String>,
}

pub struct ConstTag {
    base_node: BaseNode,
    expression: AssignmentExpression,
}

struct DebugTag {
    base_node: BaseNode,
    // identifiers: Vec<Node>
}

pub enum DirectiveTypes {
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

struct BaseDirective {
    base_node: BaseNode,
    name: String,
}

struct BaseExpressionDirective {
    base_directive: BaseDirective,
    // expression: Optionn<Node>
    name: String,
    modifiers: Vec<String>,
}

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

pub enum ElementAttributes {
    BaseDirective(BaseDirective),
    Attribute(Attribute),
    SpreadAttribute(SpreadAttribute),
}

pub struct Element {
    base_node: BaseNode,
    element_type: ElementType,
    attributes: Vec<ElementAttributes>,
}

pub struct Attribute {
    base_node: BaseNode,
    name: String,
    value: Vec<String>,
}

pub struct SpreadAttribute {
    base_node: BaseNode,
    // expression: Node
}

pub struct Transition {
    base_expression_directive: BaseExpressionDirective,
    intro: bool,
    outro: bool,
}

pub enum Directive {
    BaseDirective(BaseDirective),
    BaseExpressionDirective(BaseExpressionDirective),
    Transition(Transition),
}

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

pub struct Parser {
    template: String,
    filename: Option<String>,
    index: i32,
    //stack: Vec<Node>
    //html: Node,
    //css: Node,
    //js: Node,
    meta_tags: Vec<String>,
}

pub struct Script {
    base_node: BaseNode,
    context: String,
    content: Program,
}

pub struct Style {
    base_node: BaseNode,
    // attributes: Vec<String>, // TODO - from svelte
    // children: Vec<String>,   // TODO add CSS node types - from svelte
    content: StyleContent,
}

struct StyleContent {
    start: i32,
    end: i32,
    styles: String,
}

pub struct Ast {
    html: TemplateNode,
    css: Option<Style>,
    instance: Option<Script>,
    module: Option<Script>,
}

pub struct WarningStart {
    line: i32,
    column: i32,
    pos: Option<i32>,
}

pub struct WarningEnd {
    line: i32,
    column: i32,
}
pub struct Warnning {
    start: Option<WarningStart>,
    end: Option<WarningEnd>,
    pos: Option<i32>,
    code: String,
    message: String,
    filename: Option<String>,
    frame: Option<String>,
}

pub enum ModuleFormat {
    Esm,
    Cjs,
}

pub enum EnableSourcemap {
    Enable(bool),
    Type { js: bool, css: bool },
}

// TODO
pub enum CssHashGetter {
    // **Typescript**
    // 	export type CssHashGetter = (args: {
    // 	name: string;
    // 	filename: string | undefined;
    // 	css: string;
    // 	hash: (input: string) => string;
    // }) => string;
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
    format: Option<ModuleFormat>,
    name: Option<String>,
    filename: Option<String>,
    generate: Option<Generate>,
    error_mode: Option<ErrorMode>,
    vars_report: Option<VariablesReport>,
    sourcemap: Option<String>, // object | string
    enable_sourcemap: Option<EnableSourcemap>,
    output_filename: Option<String>,
    css_output_filename: Option<String>,
    svelte_path: Option<String>,
    dev: Option<bool>,
    accesors: Option<bool>,
    immutable: Option<bool>,
    hydratable: Option<bool>,
    legacy: Option<bool>,
    custom_element: Option<bool>,
    tag: Option<String>,
    css: Option<bool>,
    loop_guard_timenout: Option<i32>,
    namespace: Option<String>,
    css_hash: Option<CssHashGetter>,
    preserve_comments: Option<bool>,
    preserve_whitespace: Option<bool>,
}

pub struct ParserOptions {
    filename: Option<String>,
    custom_element: Option<bool>,
}

pub struct Visitor {
    // **Typescript**
    // enter: (node: Node) => void;
    // leave?: (node: Node) => void;
}

pub struct AppendTarget {
    slots: HashMap<String, String>,
    slot_stack: Vec<String>,
}

pub struct Var {
    name: String,
    export_name: Option<String>, // the `bar` in `export { foo as bar }`
    injected: Option<bool>,
    module: Option<bool>,
    mutated: Option<bool>,
    reassigned: Option<bool>,
    referenced: Option<bool>,             // referenced from template scope
    referenced_from_script: Option<bool>, // referenced from script

    // used internally, but not exposed
    global: Option<bool>,
    internal: Option<bool>, // event handlers, bindings
    initialised: Option<bool>,
    hoistable: Option<bool>,
    subscribable: Option<bool>,
    is_reactive_dependency: Option<bool>,
    imported: Option<bool>,
}

// TODO
pub struct CssResult {
    code: String,
    // magic-string SourceMap
    // map: SourceMap
}
