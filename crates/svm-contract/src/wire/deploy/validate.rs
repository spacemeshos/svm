use super::error::ContractBuildError;
use super::field::Field;

use crate::wasm::Contract;
use svm_common::Address;

pub fn validate_contract(contract: &Contract) -> Result<(), ContractBuildError> {
    validate_author(contract)?;
    validate_admins(contract)?;
    validate_deps(contract)?;
    validate_wasm(contract)?;

    Ok(())
}

fn validate_deps(_contract: &Contract) -> Result<(), ContractBuildError> {
    Ok(())
}

fn validate_author(contract: &Contract) -> Result<(), ContractBuildError> {
    validate_account(contract.author, Field::Author)
}

fn validate_admins(_contract: &Contract) -> Result<(), ContractBuildError> {
    Ok(())
}

fn validate_account(_addr: Address, _field: Field) -> Result<(), ContractBuildError> {
    Ok(())
}

fn validate_wasm(_contract: &Contract) -> Result<(), ContractBuildError> {
    Ok(())
}
