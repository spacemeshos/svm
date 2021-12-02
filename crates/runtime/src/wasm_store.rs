use wasmer::{Singlepass, Store, Universal};

/// New fresh `Store`.
#[must_use]
pub fn new_store() -> Store {
    let engine = Universal::new(Singlepass::default()).engine();
    Store::new(&engine)
}
