use crate::traits::{
    ContractAddressCompute, ContractCodeHasher, ContractDeserializer, ContractSerializer,
    ContractStore,
};
use crate::types::CodeHash;
use crate::wasm::WasmContract;
use crate::wire::ContractError;

use svm_common::{Address, KeyHasher};

pub trait ContractEnvTypes {
    type Serializer: ContractSerializer;

    type Deserializer: ContractDeserializer;

    type Store: ContractStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute: ContractAddressCompute;

    type CodeHasher: ContractCodeHasher;
}

pub trait ContractEnv {
    type Types: ContractEnvTypes;

    fn get_store(&self) -> &<Self::Types as ContractEnvTypes>::Store;

    fn get_store_mut(&mut self) -> &mut <Self::Types as ContractEnvTypes>::Store;

    fn close_store(&mut self);

    #[inline(always)]
    fn compute_code_hash(contract: &WasmContract) -> CodeHash {
        <Self::Types as ContractEnvTypes>::CodeHasher::hash(&contract.wasm)
    }

    #[inline(always)]
    fn compute_address(contract: &WasmContract) -> Address {
        <Self::Types as ContractEnvTypes>::AddressCompute::compute(contract)
    }

    fn build_contract(bytes: &[u8]) -> Result<WasmContract, ContractError> {
        let mut contract = crate::wire::parse_contract(bytes)?;

        crate::wire::validate_contract(&contract)?;

        contract.address = Some(Self::compute_address(&contract));

        Ok(contract)
    }

    #[inline(always)]
    fn store_contract(&mut self, contract: &WasmContract) {
        let hash = Self::compute_code_hash(contract);
        let store = self.get_store_mut();

        store.store(&contract, hash);
    }
}
