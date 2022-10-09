use std::{fs, path::Path};

use compiler::{analyse::analyse, generate::generate, parse::Parser};

pub mod compiler;

pub fn compile_file_to_js(input: &Path, output: &Path) -> Result<(), std::io::Error> {
    let source = fs::read_to_string(input)?;
    let ast = Parser::new(&source).parse();
    let analysis = analyse(&ast);
    let generated = generate(ast, analysis);

    fs::write(output, generated)?;

    Ok(())
}
