use wasmer::Store;

/// New fresh `Store`.
#[cfg(feature = "default-cranelift")]
#[must_use]
pub fn new_store() -> Store {
    use wasmer::{Cranelift, Universal};

    let engine = Universal::new(Cranelift::default()).engine();
    Store::new(&engine)
}

/// New fresh `Store`.
#[cfg(feature = "default-singlepass")]
#[must_use]
pub fn new_store() -> Store {
    use wasmer::{Singlepass, Universal};

    let engine = Universal::new(Singlepass::default()).engine();
    Store::new(&engine)
}
