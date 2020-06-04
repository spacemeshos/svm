use wasmer_clif_backend::CraneliftCompiler;
use wasmer_runtime_core::{error::CompileResult, Module};

// use crate::middleware::ValidationMiddleware;

// use wasmer_middleware_common::metering::Metering;
// use wasmer_runtime_core::codegen::{MiddlewareChain, StreamingCompiler};
// use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;

/// This function is responsible on compiling a wasm program using the `wasmer singlepass` compiler along
/// with the the middlewares required by `SVM`.
// #[must_use]
// pub fn compile_singlepass(
//     wasm: &[u8],
//     gas_limit: u64,
//     gas_metering: bool,
// ) -> CompileResult<Module> {
//     let compiler: StreamingCompiler<SinglePassMCG, _, _, _, _> =
//         StreamingCompiler::new(move || {
//             let mut chain = MiddlewareChain::new();

//             chain.push(ValidationMiddleware::new());

//             if gas_metering {
//                 chain.push(Metering::new(gas_limit))
//             }

//             chain
//         });

//     wasmer_runtime_core::compile_with(wasm, &compiler)
// }

#[must_use]
pub fn compile_program(wasm: &[u8], _gas_limit: u64, _gas_metering: bool) -> CompileResult<Module> {
    wasmer_runtime_core::compile_with(wasm, &CraneliftCompiler::new())
}
