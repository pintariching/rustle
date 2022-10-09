use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::{EsVersion, Script};
use swc_ecma_codegen::{text_writer::JsWriter, Config, Emitter};

pub fn print_js(script: Script) -> String {
    let mut buffer = Vec::new();
    {
        let cm: Lrc<SourceMap> = Default::default();
        let writer = JsWriter::new(cm.clone(), "\n", &mut buffer, None);
        let config = Config {
            target: EsVersion::latest(),
            ascii_only: false,
            minify: false,
            omit_last_semi: false,
        };
        let mut emmiter = Emitter {
            cfg: config,
            cm: cm.clone(),
            comments: None,
            wr: writer,
        };
        emmiter.emit_script(&script).unwrap();
    }

    String::from_utf8(buffer).unwrap()
}
