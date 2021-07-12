use wasmer::{Cranelift, Store, Universal};

/// New fresh `Store`.
#[must_use]
pub fn new_store() -> Store {
    //#[cfg(feature, "default-cranelift")]
    new_store_cranelift()
}

//#[cfg(feature, "default-cranelift")]
fn new_store_cranelift() -> Store {
    let compiler_config = Cranelift::default();
    let engine = Universal::new(compiler_config).engine();
    Store::new(&engine)
}
