use std::{fs, path::Path};

use compiler::{analyse::analyse, generate::generate, parse::Parser, RustleAst};

pub mod compiler;

pub fn compile_file_to_js(input: &Path, output: &Path) -> Result<(), std::io::Error> {
    let source = fs::read_to_string(input)?;
    let mut ast = Parser::new(&source).parse();
    let analysis = analyse(&mut ast);
    let generated = generate(ast, analysis);

    fs::write(output, generated)?;

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
    let generated = generate(ast, analysis);

    Ok(generated)
}
