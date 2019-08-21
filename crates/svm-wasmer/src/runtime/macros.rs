#[macro_export]
macro_rules! include_svm_runtime {
    ($PAGE_CACHE: ident, $CONTRACT_TYPES: ty) => {
        use crate::{include_wasmer_svm_vmcalls, runtime::Tx};

        use svm_common::{Address, State};
        use svm_contract::{build_wasm_contract, wasm::WasmContract, ContractError};

        include_wasmer_svm_vmcalls!($PAGE_CACHE);

        #[inline(always)]
        pub fn contract_build(bytes: &[u8]) -> Result<WasmContract, ContractError> {
            // 2. computes contract's address for contract using `CONTRACT_TYPES::AddressCompute` (implements `ContractAddressCompute`)
            svm_contract::build_wasm_contract::<$CONTRACT_TYPES>(&bytes)
        }

        #[inline(always)]
        pub fn contract_validate_wasm(
            contract: &WasmContract,
        ) -> Result<WasmContract, ContractError> {
            // 1. validates the `wasm`
            unimplemented!()
        }

        pub fn contract_store(contract: &WasmContract) {
            // 1. Looks if contract account exists under `CONTRACT_TYPES::Store` (import `CodeHashStore`)
            // 2. If contract exists, panics
            // 3. Else, stores contract under `CONTRACT_TYPES::Store`
            unimplemented!()
        }

        pub fn contract_exec(tx: Tx) {
            // 1. Load contract wasmer module `tx.Address`
            //  * if it's NOT in the compiled-modules-cache
            //      * Gets the wasm code from the `CONTRACT_TYPES::Store` (implements `CodeHashStore`)
            //      * Compile the module using `svm_compiler::compile_program(..)`
            //      * Store into the compiled-modules-cache
            //
            // 2. Validates that module has function `tx.FuncName` and that it can accept `tx.FuncArgs`
            //
            // 3. Build the import object with `address = tx.Address` and `state = tx.State`
            //
            // 4. Instantiate wasm instance
            //
            // 5. Get the exported function `tx.FuncName`
            //
            // 6. Execute the function with input `tx.FuncArgs`
        }
    };
}
