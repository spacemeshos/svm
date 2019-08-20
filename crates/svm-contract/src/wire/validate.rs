use super::error::Error;
use super::field::Field;

use crate::types::Revision;
use crate::wasm::WasmContract;
use svm_common::Address;

pub fn validate_contract(contract: &WasmContract) -> Result<(), Error> {
    validate_author(contract)?;
    validate_admins(contract)?;
    validate_deps(contract)?;
    validate_wasm(contract)?;

    Ok(())
}

fn validate_deps(_contract: &WasmContract) -> Result<(), Error> {
    return Ok(());
}

fn validate_author(contract: &WasmContract) -> Result<(), Error> {
    validate_account(contract.author, Field::Author)
}

fn validate_admins(_contract: &WasmContract) -> Result<(), Error> {
    Ok(())
}

fn validate_account(_addr: Address, _field: Field) -> Result<(), Error> {
    Ok(())
}

fn validate_wasm(_contract: &WasmContract) -> Result<(), Error> {
    Ok(())
}
