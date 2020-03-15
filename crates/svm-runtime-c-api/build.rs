extern crate cbindgen;
use cbindgen::{Builder, Language};
use std::{env, fs, path::PathBuf};

fn main() {
    generate_svm_header();
}

fn generate_svm_header() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let mut src_header = PathBuf::from(&out_dir);
    src_header.push("svm");
    src_header.set_extension("h");

    // build using cbindgen
    Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(Language::C)
        .with_include_guard("SVM_H")
        .with_parse_expand(&["svm-runtime-c-api"])
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(src_header.as_path());

    let mut dst_header = PathBuf::from("../../examples");
    dst_header.push("svm");
    dst_header.set_extension("h");

    // copies `svm.h` under `examples`
    fs::copy(src_header.as_path(), dst_header.as_path())
        .expect("Unable to copy the generated C bindings");
}
