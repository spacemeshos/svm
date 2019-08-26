#[macro_export]
macro_rules! include_svm_runtime {
    ($PAGE_CACHE: ident, $ENV: ty) => {
        $crate::include_wasmer_svm_vmcalls!($PAGE_CACHE);

        pub mod runtime {
            #[inline(always)]
            pub fn contract_build(
                bytes: &[u8],
            ) -> Result<svm_contract::wasm::WasmContract, svm_contract::ContractError> {
                <$ENV as svm_contract::env::ContractEnv>::build_contract(&bytes)
            }

            #[inline(always)]
            pub fn contract_validate(
                contract: &svm_contract::wasm::WasmContract,
            ) -> Result<(), svm_contract::ContractError> {
                // validates the `wasm`. should use the `deterministic` feature of `wasmparser`.
                // (avoiding floats)
                unimplemented!()
            }

            #[inline(always)]
            pub fn contract_store(contract: &svm_contract::wasm::WasmContract, env: &mut $ENV) {
                use svm_contract::env::ContractEnv;
                env.store_contract(&contract)
            }

            pub fn contract_exec(tx: $crate::runtime::Tx) {
                use svm_common::{Address, State};

                // 1. Loads contract wasmer module `tx.Address`
                //  * if it's NOT in the compiled-modules-cache
                //      * Gets the wasm code from the `ENV::Store` (implements `CodeHashStore`)
                //      * Compile the module using `svm_compiler::compile_program(..)`
                //      * Store into the compiled-modules-cache
                //
                // 2. Validates that module has function `tx.FuncName` and that it can accept `tx.FuncArgs`
                //
                // 3. Builds the import object with `address = tx.Address` and `state = tx.State`
                //
                // 4. Instantiate wasm instance
                //
                // 5. Get the exported function `tx.FuncName`
                //
                // 6. Execute the function with input `tx.FuncArgs`
            }
        }
    };
}
