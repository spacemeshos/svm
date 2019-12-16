use wasmer_runtime_core::error::CompileResult;
use wasmer_runtime_core::Module;

use crate::middleware::ValidationMiddleware;
use wasmer_runtime_core::codegen::{MiddlewareChain, StreamingCompiler};
use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;

/// This function is responsible on compiling a wasm program using the `wasmer singlepass` compiler along
/// with the the middlewares required by `SVM`.
#[must_use]
pub fn compile_program(wasm: &[u8]) -> CompileResult<Module> {
    let compiler: StreamingCompiler<SinglePassMCG, _, _, _, _> =
        StreamingCompiler::new(move || {
            let mut chain = MiddlewareChain::new();
            chain.push(ValidationMiddleware::new());
            chain
        });

    wasmer_runtime_core::compile_with(wasm, &compiler)
}
