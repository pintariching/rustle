use std::fs;
use std::path::Path;

use rustle::compile_file_to_js;
use rustle::compiler::{analyse, generate_css, generate_js, Parser};

#[test]
fn test_parsing() {
    let source = fs::read_to_string("tests/app.svelte").unwrap();
    let mut ast = Parser::new(&source).parse();

    let analysis = analyse(&mut ast);

    // println!("Variables: {:#?}", analysis.variables);
    // println!("Will change: {:#?}", analysis.will_change);
    // println!("Will use in template: {:#?}", analysis.will_use_in_template);
    // println!("CSS classes: {:#?}", analysis.css_classes);
    // println!(
    //     "CSS classes in template: {:#?}",
    //     analysis.css_classes_in_template
    // );

    let js = generate_js(&mut ast, &analysis);
    let css = generate_css(&mut ast, &analysis);

    fs::write("tests/app.js", js).unwrap();
    fs::write("tests/app.css", css).unwrap();

    assert!(true)
}

#[test]
fn test_compile_to_js() {
    let input = Path::new("tests/app.svelte");
    let output = Path::new("tests/app.js");

    compile_file_to_js(input, output).unwrap();
}
