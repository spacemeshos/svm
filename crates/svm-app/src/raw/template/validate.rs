use crate::{error::ParseError, types::AppTemplate};

/// Validates an app-template.
/// Should be called right after parsing the raw bytes into `AppTemplate`.
#[allow(dead_code)]
pub fn validate_template(_template: &AppTemplate) -> Result<(), ParseError> {
    todo!()
}
