use std::ffi::c_void;

use crate::{contract_settings::ContractSettings, Receipt};

use svm_common::{Address, State};
use svm_contract::{
    error::{ContractBuildError, TransactionBuildError},
    transaction::Transaction,
    wasm::Contract,
};
use svm_storage::ContractStorage;

use wasmer_runtime_core::{export::Export, import::ImportObject};

pub trait Runtime {
    fn contract_build(&self, bytes: &[u8]) -> Result<Contract, ContractBuildError>;

    fn contract_derive_address(&self, contract: &Contract) -> Address;

    fn contract_deploy(&mut self, contract: &Contract, addr: &Address);

    fn transaction_build(&self, bytes: &[u8]) -> Result<Transaction, TransactionBuildError>;

    fn transaction_exec(
        &self,
        tx: &Transaction,
        state: &State,
        settings: &ContractSettings,
    ) -> Receipt;
}

pub type StorageBuilderFn = dyn Fn(&Address, &State, &ContractSettings) -> ContractStorage;
