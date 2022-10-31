use swc_css_ast::Stylesheet;
use swc_css_codegen::{
    writer::basic::{BasicCssWriter, BasicCssWriterConfig},
    CodeGenerator, CodegenConfig, Emit,
};

pub fn generate_css_from_stylesheet(stylesheet: &Stylesheet) -> String {
    let mut css_str = String::new();
    {
        let wr = BasicCssWriter::new(&mut css_str, None, BasicCssWriterConfig::default());
        let mut gen = CodeGenerator::new(wr, CodegenConfig { minify: false });

        gen.emit(&stylesheet).unwrap();
    }

    css_str
}
