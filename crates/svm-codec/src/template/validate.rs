use svm_types::Template;

use crate::error::ParseError;

/// Validates an app-template.
/// Should be called right after parsing the raw bytes into `AppTemplate`.
#[allow(dead_code)]
pub fn validate_template(_template: &Template) -> Result<(), ParseError> {
    todo!()
}
