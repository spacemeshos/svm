use super::BetterConversionToJson;

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq)]
pub enum JsonError {
    InvalidJson,
    InvalidField { field: String },
}

impl JsonError {
    /// Creates a new [`JsonError`] that mirrors a [`serde_json::Error`].
    /// `expected_type` offers some error details that `serde_json` does not
    /// expose (e.g. "string", "number", "array").
    pub(crate) fn from_serde<T>(serde_err: serde_json::Error) -> Self
    where
        T: BetterConversionToJson,
    {
        let err_s = serde_err.to_string();

        if err_s.starts_with("missing field") {
            let missing_field = err_s.split('`').nth(1).unwrap();
            JsonError::InvalidField {
                field: missing_field.to_string(),
            }
        } else if serde_err.is_syntax() {
            Self::InvalidJson
        } else if serde_err.is_data() {
            Self::InvalidField {
                field: serde_err.to_string(),
            }
        } else {
            Self::InvalidJson
        }
    }
}
