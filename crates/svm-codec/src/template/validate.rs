use svm_types::Template;

use crate::ParseError;

/// Validates a `Template`.
/// Should be called right after parsing the raw bytes into `Template`.
#[allow(dead_code)]
pub fn validate_template(_template: &Template) -> Result<(), ParseError> {
    todo!()
}
