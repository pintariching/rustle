use swc_visit::define;

use self::extract_variables::extract_root_variables;

use super::RustleAst;
use std::collections::HashSet;
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};

mod extract_variables;

#[derive(Debug)]
pub struct Result {
    pub variables: HashSet<String>,
    pub will_change: HashSet<String>,
    pub will_use_in_template: HashSet<String>,
}

pub fn analyse(ast: &RustleAst) -> Result {
    let mut result = Result {
        variables: HashSet::new(),
        will_change: HashSet::new(),
        will_use_in_template: HashSet::new(),
    };

    let variables = extract_root_variables(&ast.script);

    result.variables = HashSet::from_iter(variables);

    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::compiler::parse::Parser;

    use super::analyse;

    #[test]
    fn test_analyse() {
        let source = fs::read_to_string("./tests/app.rustle").unwrap();
        let ast = Parser::new(&source).parse();
        let result = analyse(&ast);
    }
}
