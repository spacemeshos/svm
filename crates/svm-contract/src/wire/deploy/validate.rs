use super::error::ContractDeployError;
use super::field::Field;

use crate::wasm::WasmContract;
use svm_common::Address;

pub fn validate_contract(contract: &WasmContract) -> Result<(), ContractDeployError> {
    validate_author(contract)?;
    validate_admins(contract)?;
    validate_deps(contract)?;
    validate_wasm(contract)?;

    Ok(())
}

fn validate_deps(_contract: &WasmContract) -> Result<(), ContractDeployError> {
    return Ok(());
}

fn validate_author(contract: &WasmContract) -> Result<(), ContractDeployError> {
    validate_account(contract.author, Field::Author)
}

fn validate_admins(_contract: &WasmContract) -> Result<(), ContractDeployError> {
    Ok(())
}

fn validate_account(_addr: Address, _field: Field) -> Result<(), ContractDeployError> {
    Ok(())
}

fn validate_wasm(_contract: &WasmContract) -> Result<(), ContractDeployError> {
    Ok(())
}
