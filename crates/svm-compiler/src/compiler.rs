use wasmer::{Cranelift, Module, Store, JIT};
use wasmer_compiler::CompileError;

/// Compiles the SVM app
///
// TODO:
// ====
// * inject `gas-metering` middleware.
// * inject the validation middleware.
#[must_use]
pub fn compile_program(
    wasm: &[u8],
    _gas_limit: u64,
    _gas_metering: bool,
) -> Result<Module, CompileError> {
    let engine = JIT::new(&Cranelift::default()).engine();
    let store = Store::new(&engine);

    Module::from_binary(&store, wasm)
}
