use wasmer::{Module, Store, JIT};
use wasmer_compiler::CompileError;

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
    #[cfg(windows)]
    let engine = JIT::new(&wasmer::Cranelift::default()).engine();

    #[cfg(unix)]
    let engine = JIT::new(&wasmer::Singlepass::default()).engine();

    Store::new(&engine)
}
