use crate::traits::{
    ContractAddressCompute, ContractCodeHasher, ContractDeserializer, ContractSerializer,
    ContractStore,
};
use crate::transaction::Transaction;
use crate::types::CodeHash;
use crate::wasm::Contract;
use crate::wire::{deploy::ContractBuildError, exec::TransactionBuildError};

use svm_common::Address;

/// Aggregates types that are required by `ContractEnv`
pub trait ContractEnvTypes {
    /// Serializing a contract
    type Serializer: ContractSerializer;

    /// Deserializing a contract
    type Deserializer: ContractDeserializer;

    /// Storing / Loaing a contract
    type Store: ContractStore<Self::Serializer, Self::Deserializer>;

    /// Deriving the contract address
    type AddressCompute: ContractAddressCompute;

    /// Deriving the Hash of the contract code
    type CodeHasher: ContractCodeHasher;
}

/// Trait for managing the contract environment.
/// Relies on associated `ContractEnvTypes`.
pub trait ContractEnv {
    /// Contract environment is dictated by its `Types`
    type Types: ContractEnvTypes;

    /// Borrows environment's store
    fn get_store(&self) -> &<Self::Types as ContractEnvTypes>::Store;

    /// Borrows mutably environment's store
    fn get_store_mut(&mut self) -> &mut <Self::Types as ContractEnvTypes>::Store;

    /// Computes contract hash
    #[inline(always)]
    fn compute_code_hash(&self, contract: &Contract) -> CodeHash {
        <Self::Types as ContractEnvTypes>::CodeHasher::hash(&contract.wasm)
    }

    /// Computes contract account address
    #[inline(always)]
    fn compute_address(&self, contract: &Contract) -> Address {
        <Self::Types as ContractEnvTypes>::AddressCompute::compute(contract)
    }

    /// * Parses a raw contract into `Contract`
    /// * Enriches the contract with its derived address
    fn build_contract(&self, bytes: &[u8]) -> Result<Contract, ContractBuildError> {
        let contract = crate::wire::deploy::parse_contract(bytes)?;

        crate::wire::deploy::validate_contract(&contract)?;

        Ok(contract)
    }

    /// Parses a raw transaction
    fn build_transaction(&self, bytes: &[u8]) -> Result<Transaction, TransactionBuildError> {
        let tx = crate::wire::exec::parse_transaction(bytes)?;

        Ok(tx)
    }

    /// Stores contract by its `CodeHash`
    #[inline(always)]
    fn store_contract(&mut self, contract: &Contract, addr: &Address) {
        let hash = self.compute_code_hash(contract);
        let store = self.get_store_mut();

        store.store(contract, addr, hash);
    }
}
