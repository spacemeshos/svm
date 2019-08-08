use super::contract::WireContract;
use super::field::Field;
use crate::types::Revision;
use svm_common::Address;

#[allow(dead_code)]
pub enum ValidateError {
    NoAuthors,
    DepsNotSupportedYet,
    AdminsNotSupportedYet,
    InvalidWasm,
    MissingField(Field),
    DependencyNotFound(Revision),
    UnsupportedProtoVersion(u32),
}

#[allow(dead_code)]
pub fn validate_contract(contract: &WireContract) -> Result<(), ValidateError> {
    validate_author(&contract)?;
    validate_admins(&contract)?;
    validate_deps(&contract)?;
    validate_wasm(&contract)?;

    Ok(())
}

fn validate_deps(_contract: &WireContract) -> Result<(), ValidateError> {
    return Ok(());
}

fn validate_author(contract: &WireContract) -> Result<(), ValidateError> {
    validate_account(contract.author, Field::Author)
}

fn validate_admins(_contract: &WireContract) -> Result<(), ValidateError> {
    Ok(())
}

fn validate_account(_addr: Address, _field: Field) -> Result<(), ValidateError> {
    Ok(())
}

fn validate_wasm(_contract: &WireContract) -> Result<(), ValidateError> {
    Ok(())
}
