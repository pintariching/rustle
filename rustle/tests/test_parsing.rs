use std::fs;

use rustle::compiler::analyse::analyse;
use rustle::compiler::parse::Parser;

#[test]
fn test_parsing() {
    let source = fs::read_to_string("tests/app.rustle").unwrap();
    let ast = Parser::new(&source).parse();
    let analysis = analyse(&ast);

    println!("{:#?}", analysis);

    fs::write(
        "tests/app.json",
        serde_json::to_string_pretty(&ast).unwrap(),
    )
    .unwrap();

    assert!(true)
}
