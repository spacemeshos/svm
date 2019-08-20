macro_rules! include_svm_runtime {
    ($PAGE_CACHE: ident, $CONTRACT_TYPES: ty) => {{
        use crate::{include_wasmer_svm_vmcalls, Tx};

        use svm_common::{Address, State};
        use svm_contract::{build_wasm_contract, wasm::WasmContract, ContractError};

        include_wasmer_svm_vmcalls!($PAGE_CACHE);

        #[inline(always)]
        pub fn contract_build(bytes: &[u8]) -> Result<WasmContract, ContractError> {
            svm_contract::build_wasm_contract::<$CONTRACT_TYPES>(&bytes)
        }

        pub fn contract_store(contract: &WasmContract) {
            unimplemented!()
        }

        pub fn contract_exec(tx: Tx) {
            // 1. load contract wasmer module `tx.Address`
            //  * if it's NOT in the compiled-modules-cache
            //      * Gets the wasm code from the `CONTRACT_TYPES::Store` (implements `CodeHashStore`)
            //      * Compiles the module
            //      * Store into the compiled-modules-cache
            //
            // 2. build the import object with `address = tx.Address` and `state = tx.State`
            //
            // 3. instantiate wasm instance
            //
            // 4. Get the exported function `tx.FuncName`
            //
            // 5. execute the function with input `tx.FuncArgs`
        }
    }};
}
