use std::ffi::c_void;
use std::ops::Deref;

use crate::{contract_settings::ContractSettings, Receipt};

use svm_common::{Address, State};
use svm_contract::{
    error::{ContractBuildError, TransactionBuildError},
    transaction::Transaction,
    wasm::Contract,
};
use svm_storage::ContractStorage;

use wasmer_runtime_core::import::ImportObject;

pub trait Runtime: Deref<Target = c_void> {
    fn contract_build(&self, bytes: &[u8]) -> Result<Contract, ContractBuildError>;

    fn contract_derive_address(&self, contract: &Contract) -> Address;

    fn contract_deploy(&mut self, contract: &Contract, addr: &Address);

    fn transaction_build(&self, bytes: &[u8]) -> Result<Transaction, TransactionBuildError>;

    fn transaction_exec(&self, tx: &Transaction, import_object: &ImportObject) -> Receipt;
}

pub type StorageBuilderFn = dyn Fn(&Address, &State, &ContractSettings) -> ContractStorage;
