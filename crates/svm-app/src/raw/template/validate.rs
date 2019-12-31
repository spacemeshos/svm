use crate::{error::ParseError, raw::Field, types::AppTemplate};

use svm_common::Address;

pub fn validate_template(template: &AppTemplate) -> Result<(), ParseError> {
    validate_author(template)?;
    validate_admins(template)?;
    validate_deps(template)?;
    validate_wasm(template)?;

    Ok(())
}

fn validate_deps(_template: &AppTemplate) -> Result<(), ParseError> {
    Ok(())
}

fn validate_author(template: &AppTemplate) -> Result<(), ParseError> {
    validate_account(&template.author, Field::Author)
}

fn validate_admins(_template: &AppTemplate) -> Result<(), ParseError> {
    Ok(())
}

fn validate_account(_addr: &Address, _field: Field) -> Result<(), ParseError> {
    Ok(())
}

fn validate_wasm(_template: &AppTemplate) -> Result<(), ParseError> {
    Ok(())
}
