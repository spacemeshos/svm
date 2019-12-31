use super::{error::AppTemplateBuildError, field::Field};

use crate::wasm::AppTemplate;

use svm_common::Address;

pub fn validate_contract(template: &AppTemplate) -> Result<(), AppTemplateBuildError> {
    validate_author(template)?;
    validate_admins(template)?;
    validate_deps(template)?;
    validate_wasm(template)?;

    Ok(())
}

fn validate_deps(_template: &AppTemplate) -> Result<(), AppTemplateBuildError> {
    Ok(())
}

fn validate_author(template: &AppTemplate) -> Result<(), AppTemplateBuildError> {
    validate_account(&template.author, Field::Author)
}

fn validate_admins(_template: &AppTemplate) -> Result<(), AppTemplateBuildError> {
    Ok(())
}

fn validate_account(_addr: &Address, _field: Field) -> Result<(), AppTemplateBuildError> {
    Ok(())
}

fn validate_wasm(_template: &AppTemplate) -> Result<(), AppTemplateBuildError> {
    Ok(())
}
