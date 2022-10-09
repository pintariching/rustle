use swc_ecma_ast::{Program, EsVersion, Expr};
use swc_common::source_map::{SourceFile, FileName, BytePos};
use swc_ecma_parser::{EsConfig, Syntax, error::Error};

mod print;
mod utils;


pub fn parse(source: String) -> Result<Program, Error> {

    //Add an alternative to estree walker and acorns options later on
    let mut errors: Vec<Error> = Vec::new();
    let source_file = SourceFile::new(FileName::Custom("<rustle>".into()), false, FileName::Custom("<rustle>".into()), source, BytePos(0));
    let parse_results = swc_ecma_parser::parse_file_as_program(&source_file,Syntax::Es(EsConfig::default()) , EsVersion::latest(), None, &mut errors);

    parse_results
}

pub fn parse_expression_at(source: String, index: usize) -> Result<Box<Expr>, Error> {
    let mut errors: Vec<Error> = Vec::new();
    //Add an alternative to estree walker and acorns options later on
    let source_file = SourceFile::new(FileName::Custom("<rustle>".into()), false, FileName::Custom("<rustle>".into()), source, BytePos(index));
    let parse_results =  swc_ecma_parser::parse_file_as_expr(&source_file, Syntax::Es(EsConfig::default()), EsVersion::latest(), None, &mut errors);

    parse_results
}
