use svm_types::AppTemplate;

use crate::error::ParseError;

/// Validates an app-template.
/// Should be called right after parsing the raw bytes into `AppTemplate`.
#[allow(dead_code)]
pub fn validate_template(_template: &AppTemplate) -> Result<(), ParseError> {
    todo!()
}
