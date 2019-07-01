/// The `svm_compiler` macro returns a `wasmer single pass compiler` with middlewares required by the `svm`
#[macro_export]
macro_rules! svm_compiler {
    () => {{
        use crate::middleware::ValidationMiddleware;

        use wasmer_runtime_core::backend::RunnableModule;
        use wasmer_runtime_core::codegen::{
            MiddlewareChain, SimpleStreamingCompilerGen, StreamingCompiler,
        };

        use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;

        // since we can't say explicitly all the wildcards (`_`) we can't a function
        // returning a `StreamingCompiler<SinglePassMCG, _, _, _, _>` so we use a rust macro instead
        let compiler: StreamingCompiler<SinglePassMCG, _, _, _, _> =
            StreamingCompiler::new(move || {
                let mut chain = MiddlewareChain::new();
                chain.push(ValidationMiddleware::new());
                chain
            });

        compiler
    }};
}
