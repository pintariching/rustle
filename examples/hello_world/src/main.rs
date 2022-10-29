use std::path::Path;

fn main() {
    let input = Path::new("./app.svelte");
    let output = Path::new("./app.js");
    rustle::compile_file_to_js(input, output).unwrap();
}
