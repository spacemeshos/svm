use crate::traits::{
    ContractAddressCompute, ContractCodeHasher, ContractDeserializer, ContractSerializer,
    ContractStore,
};
use crate::transaction::Transaction;
use crate::types::CodeHash;
use crate::wasm::Contract;
use crate::wire::{deploy::ContractBuildError, exec::TransactionBuildError};

use svm_common::Address;

pub trait ContractEnvTypes {
    type Serializer: ContractSerializer;

    type Deserializer: ContractDeserializer;

    type Store: ContractStore<Self::Serializer, Self::Deserializer>;

    type AddressCompute: ContractAddressCompute;

    type CodeHasher: ContractCodeHasher;
}

/// Trait for managing the contract environment.
/// Relies on associated `ContractEnvTypes`.
pub trait ContractEnv {
    type Types: ContractEnvTypes;

    /// Borrows environment's store
    fn get_store(&self) -> &<Self::Types as ContractEnvTypes>::Store;

    /// Borrows mutably environment's store
    fn get_store_mut(&mut self) -> &mut <Self::Types as ContractEnvTypes>::Store;

    /// Computes contract hash
    #[inline(always)]
    fn compute_code_hash(contract: &Contract) -> CodeHash {
        <Self::Types as ContractEnvTypes>::CodeHasher::hash(&contract.wasm)
    }

    /// Computes contract account address
    #[inline(always)]
    fn compute_address(contract: &Contract) -> Address {
        <Self::Types as ContractEnvTypes>::AddressCompute::compute(contract)
    }

    /// * Parses a raw contract into `Contract`
    /// * Enriches the contract with its derived address
    fn build_contract(bytes: &[u8]) -> Result<Contract, ContractBuildError> {
        let mut contract = crate::wire::deploy::parse_contract(bytes)?;

        crate::wire::deploy::validate_contract(&contract)?;

        contract.address = Some(Self::compute_address(&contract));

        Ok(contract)
    }

    /// Parses a raw transaction
    fn build_transaction(bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
        let tx = crate::wire::exec::parse_transaction(bytes)?;

        Ok(tx)
    }

    /// Stores contract by its `CodeHash`
    #[inline(always)]
    fn store_contract(&mut self, contract: &Contract) {
        let hash = Self::compute_code_hash(contract);
        let store = self.get_store_mut();

        store.store(&contract, hash);
    }
}
