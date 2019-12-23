use crate::{contract_settings::ContractSettings, Receipt};

use svm_common::{Address, State};
use svm_contract::{
    error::{ContractBuildError, TransactionBuildError},
    transaction::Transaction,
    wasm::Contract,
};
use svm_storage::ContractStorage;

/// Specifies the interface of a Smart-Contracts Runtime.
pub trait Runtime {
    /// Given a deploy-contract raw network payload, builds a `Contract` struct.
    fn contract_build(&self, bytes: &[u8]) -> Result<Contract, ContractBuildError>;

    /// Derives the contract address.
    fn contract_derive_address(&self, contract: &Contract) -> Address;

    /// Deploys the contract to a `ContractStore`.
    fn contract_deploy(&mut self, contract: &Contract, addr: &Address);

    /// Given a contract-transaction raw a network payload, builds a `Transaction` struct.
    fn transaction_build(&self, bytes: &[u8]) -> Result<Transaction, TransactionBuildError>;

    /// Executes a contract-transaction. Returns a `Receipt`.
    /// On success:
    /// * Persists changes to the contract's own storage.
    /// * Receipt returns the new contract storage state
    /// * Receipt informs the amount of gas used
    ///
    /// On failure:
    /// * Receipt returns the occurred error
    /// * Receipt informs the amount of gas used (transaction gas limit)
    fn transaction_exec(
        &self,
        tx: &Transaction,
        state: &State,
        settings: &ContractSettings,
    ) -> Receipt;
}

/// Represents a function that builds a `ContractStorage` given its address, state and settings.
pub type StorageBuilderFn = dyn Fn(&Address, &State, &ContractSettings) -> ContractStorage;
