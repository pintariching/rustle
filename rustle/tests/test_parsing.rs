use std::fs;
use std::path::Path;

use rustle::compile_file_to_js;
use rustle::compiler::analyse::analyse;
use rustle::compiler::generate::generate;
use rustle::compiler::parse::Parser;

#[test]
fn test_parsing() {
    let source = fs::read_to_string("tests/app.svelte").unwrap();
    let ast = Parser::new(&source).parse();
    let analysis = analyse(&ast);

    let generated = generate(ast, analysis);

    fs::write("tests/app.js", generated).unwrap();

    assert!(true)
}

#[test]
fn test_compile_to_js() {
    let input = Path::new("tests/app.svelte");
    let output = Path::new("tests/app.js");

    compile_file_to_js(input, output).unwrap();
}
