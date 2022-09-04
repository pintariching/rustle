use lazy_static::lazy_static;
use std::collections::HashMap;
use swc_estree_ast::{BaseNode, Comment};

lazy_static! {
    // This is an object in js, not sure how to map it to rust
    // due to `in` and `instanceof`
    pub static ref OPERATOR_PRECEDENCE: HashMap<&'static str, usize> = HashMap::from([
        ("||", 2),
        ("&&", 3),
        ("??", 4),
        ("|", 5),
        ("^", 6),
        ("&", 7),
        ("==", 8),
        ("!=", 8),
        ("===", 8),
        ("!==", 8),
        ("<", 9),
        (">", 9),
        ("<=", 9),
        (">=", 9),
        ("in", 9),
        ("instanceof", 9),
        ("<<", 10),
        (">>", 10),
        (">>>", 10),
        ("+", 11),
        ("-", 11),
        ("*", 12),
        ("%", 12),
        ("/", 12),
        ("**", 13),
    ]);

    pub static ref EXPRESSIONS_PRECEDENCE: HashMap<&'static str, usize> = HashMap::from([
        ("ArrayExpression", 20),
        ("TaggedTemplateExpression", 20),
        ("ThisExpression", 20),
        ("Identifier", 20),
        ("Literal", 18),
        ("TemplateLiteral", 20),
        ("Super", 20),
        ("SequenceExpression", 20),
        ("MemberExpression", 19),
        ("CallExpression", 19),
        ("NewExpression", 19),
        ("AwaitExpression", 17),
        ("ClassExpression", 17),
        ("FunctionExpression", 17),
        ("ObjectExpression", 17),
        ("UpdateExpression", 16),
        ("UnaryExpression", 15),
        ("BinaryExpression", 14),
        ("LogicalExpression", 13),
        ("ConditionalExpression", 4),
        ("ArrowFunctionExpression", 3),
        ("AssignmentExpression", 3),
        ("YieldExpression", 2),
        ("RestElement", 1)
    ]);
}

pub struct Chunk {
    content: String,
    location: Option<Location>,
    has_newline: bool,
}

pub struct Location {
    start: Position,
    end: Position,
}

pub struct Position {
    line: usize,
    column: usize,
}

pub struct State {
    indent: String,
    scope: String,
    scope_map: HashMap<BaseNode, String>,
    deconflicted: HashMap<BaseNode, HashMap<String, String>>,
    comments: Vec<Comment>,
}

impl State {
    fn get_name(name: String) -> String {
        todo!()
    }
}

pub fn handle(node: BaseNode, state: State) -> Vec<Chunk> {
    todo!()
}
