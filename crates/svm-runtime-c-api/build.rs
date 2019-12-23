extern crate cbindgen;
use cbindgen::{Builder, Language};
use std::{env, fs, path::PathBuf};

fn main() {
    gen_for_c();
}
fn gen_for_c() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let header_name: &str = "svm";

    // set expand dir for macro expanding
    env::set_var("CARGO_EXPAND_TARGET_DIR", crate_dir.clone());

    // set target ouput dir for header
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut out_header = PathBuf::from(&out_dir).join("../../../");
    out_header.push(header_name);
    out_header.set_extension("h");

    // build using cbindgen
    Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(Language::C)
        .with_include_guard("WASMER_SVM_H")
        .with_header("#include \"wasmer.h\"")
        .with_parse_expand(&["svm-runtime-c-api"])
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(out_header.as_path());

    // set example dir for header to copy into
    let out_path = PathBuf::from("./examples");
    let mut crate_header = PathBuf::from(&out_path);
    crate_header.push(header_name);

    // copy the file from output to example
    crate_header.set_extension("h");
    out_header.set_extension("h");
    fs::copy(out_header.as_path(), crate_header.as_path())
        .expect("Unable to copy the generated C bindings");
}
