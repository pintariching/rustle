use crate::compiler::interfaces::TemplateNode;

struct LastAutoClosedTag {
    tag: String,
    reason: String,
    depth: i32,
}

pub struct Parser {
    template: String,
    filename: Option<String>,
    custom_element: bool,
    index: i32,
    stack: Vec<TemplateNode>,
    html: Fragment,
    css: Vec<Style>,
    js: Vec<Script>,
    meta_tags: Vec<String>,
    last_auto_closed_tag: Option<LastAutoClosedTag>,
}
