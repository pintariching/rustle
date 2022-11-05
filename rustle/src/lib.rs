use std::{fs, path::Path};

use compiler::{analyse, generate_js, Fragment, Parser, RustleAst};

pub mod compiler;

pub fn compile_file_to_js(input: &Path, output: &Path) -> Result<(), std::io::Error> {
    let source = fs::read_to_string(input)?;
    let mut ast = Parser::new(&source).parse();
    let analysis = analyse(&mut ast);
    let generated = generate_js(&mut ast, &analysis);

    fs::write(output, generated)?;

    // check if file contains any nested components
    if ast.fragments.iter().any(|f| match f {
        Fragment::Element(e) => e.nested_component,
        _ => false,
    }) {
        todo!()
    }

    Ok(())
}

pub fn parse_file(input: &Path) -> Result<RustleAst, std::io::Error> {
    let source = fs::read_to_string(input)?;
    let ast = Parser::new(&source).parse();

    Ok(ast)
}

pub fn compile_file_to_string(input: &Path) -> Result<String, std::io::Error> {
    let source = fs::read_to_string(input)?;
    let mut ast = Parser::new(&source).parse();
    let analysis = analyse(&mut ast);
    let generated = generate_js(&mut ast, &analysis);

    Ok(generated)
}
