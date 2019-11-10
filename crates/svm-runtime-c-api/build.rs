extern crate cbindgen;
use cbindgen::{Builder, Language};
use std::{env, path::PathBuf};

fn main() {
    gen_for_c();
}
fn gen_for_c() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    env::set_var("CARGO_EXPAND_TARGET_DIR", crate_dir.clone());
    let out_path = PathBuf::from("./examples");

    Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(Language::C)
        .with_include_guard("WASMER_SVM_H")
        .with_header("#include \"wasmer.h\"")
        .with_parse_expand(&["svm-runtime-c-api"])
        .generate()
        .expect("Unable to generate C bindings")
        .write_to_file(out_path.join("svm_wasmer.h"));
}
