use svm_gas::{FuncPrice, ProgramVisitor};
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
    gas_limit: u64,
    gas_metering: bool,
) -> Result<Module, CompileError> {
    if gas_metering {
        let _prices = calculate_gas_limit(wasm, gas_limit);
        return Err(CompileError::Validate("Insufficient gas.".to_string()));
    }
    Module::from_binary(&store, wasm)
}

fn calculate_gas_limit(wasm: &[u8], _gas_limit: u64) -> FuncPrice {
    let program = svm_gas::read_program(wasm).unwrap();
    let pricing_resolver = svm_gas::resolvers::ExampleResolver::default();
    let program_pricing = svm_gas::ProgramPricing::new(pricing_resolver);

    let prices = program_pricing.visit(&program).unwrap();
    prices
}

/// New fresh `Store`
#[must_use]
pub fn new_store() -> Store {
    let compiler_config = Cranelift::default();
    let engine = Universal::new(compiler_config).engine();

    Store::new(&engine)
}
