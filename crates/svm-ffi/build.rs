use cbindgen::{Builder, Language};

use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // targeting the workspace 'target/(debug|release) dir
    let mut src_header = PathBuf::from(&out_dir);
    src_header.pop();
    src_header.pop();
    src_header.pop();
    src_header.push("svm_types");
    src_header.set_extension("h");

    // build using cbindgen
    Builder::new()
        .with_language(Language::C)
        .with_crate(crate_dir.clone())
        .with_include_guard("SVM_TYPES_H")
        .include_item("svm_byte_array")
        .include_item("svm_trap_t")
        .include_item("svm_func_callback_t")
        .include_item("svm_env_t")
        .with_documentation(true)
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(src_header.as_path());
}
