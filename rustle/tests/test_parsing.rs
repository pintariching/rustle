use std::fs;

use rustle::compiler::analyse::analyse;
use rustle::compiler::generate::generate;
use rustle::compiler::parse::Parser;

#[test]
fn test_parsing() {
    let source = fs::read_to_string("tests/app.rustle").unwrap();
    let ast = Parser::new(&source).parse();
    let analysis = analyse(&ast);
    let generated = generate(ast, analysis);

    fs::write("tests/app.js", generated).unwrap();

    assert!(true)
}
