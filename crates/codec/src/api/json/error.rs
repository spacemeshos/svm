#[doc(hidden)]
#[derive(Debug, PartialEq, Eq)]
pub enum JsonError {
    /// JSON syntax error.
    InvalidJson {
        line: usize,
        column: usize,
    },
    MissingField {
        field_name: String,
    },
    InvalidField {
        path: String,
    },
}

impl From<serde_path_to_error::Error<serde_json::Error>> for JsonError {
    fn from(err: serde_path_to_error::Error<serde_json::Error>) -> Self {
        if err.inner().is_data() {
            let path_of_error = err.path().to_string();
            let serde_json_err = err.inner().to_string();

            if serde_json_err.starts_with("missing field") {
                let field_name = serde_json_err.split('`').nth(1).unwrap().to_string();
                Self::MissingField { field_name }
            } else {
                JsonError::InvalidField {
                    path: path_of_error,
                }
            }
        } else {
            Self::InvalidJson {
                line: err.inner().line(),
                column: err.inner().column(),
            }
        }
    }
}
