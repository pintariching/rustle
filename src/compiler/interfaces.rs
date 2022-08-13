use chumsky::prelude::*;
use swc_estree_ast::{AssignmentExpression, BaseNode, Program};

pub struct BaseNode {
    start: i32,
    end: i32,
    node_type: String,
    children: Option<Vec<TemplateNode>>,
    prop_names: Vec<String>,
}

pub trait BaseNodeTrait<T> {
    fn new() -> T;
}

pub struct Fragment {
    base_node: BaseNode,
}

impl BaseNodeTrait<Fragment> for Fragment {
    fn new() -> Fragment {
        Fragment { base_node }
    }
}

pub struct Text {
    base_node: BaseNode,
}

impl BaseNodeTrait<Text> {}

pub enum TemplateNode {
    Text,
    ConstTag,
    DebugTag,
    MustacheTag,
    BaseNode,
    Element,
    Attribute,
    SpreadAttribute,
    Directive,
    Transition,
    Comment,
}

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

pub struct Parser {
    template: String,
    filename: Option<String>,
    index: i32,
    stack: Vec<Node>,
    html: Node,
    css: Node,
    js: Node,
    meta_tags: Vec<String>,
}
