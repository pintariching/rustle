use std::collections::HashMap;

use swc_estree_ast::{BaseNode, Comment};

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
