use wasmer::Store;

/// New fresh `Store`.
#[must_use]
pub fn new_store() -> Store {
    // By importing items within `if` blocks rather than at the top of the file,
    // we don't run the risk of importing stuff that specific features don't
    // use (which would cause compiler warnings).
    if cfg!(feature = "default-cranelift") {
        use wasmer::{Cranelift, Universal};

        let compiler_config = Cranelift::default();
        let engine = Universal::new(compiler_config).engine();
        Store::new(&engine)
    } else {
        panic!("No Wasmer backend available.");
    }
}
