use wasmer::{CompileError, Cranelift, Module, Store, Universal};

/// Compiles the SVM app
///
// TODO:
// ====
// * inject `gas-metering` middleware.
// * inject the validation middleware.
#[must_use]
pub fn compile(
    store: &Store,
    wasm: &[u8],
    _gas_limit: u64,
    _gas_metering: bool,
) -> Result<Module, CompileError> {
    Module::from_binary(&store, wasm)
}

/// New fresh `Store`
#[must_use]
pub fn new_store() -> Store {
    let compiler_config = Cranelift::default();
    let engine = Universal::new(compiler_config).engine();

    Store::new(&engine)
}
