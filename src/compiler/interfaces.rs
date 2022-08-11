pub struct BaseNode {
    start: i32,
    end: i32,
    node_type: String,
    children: Option<Vec<TemplateNode>>,
    prop_names: Vec<String>,
}

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
