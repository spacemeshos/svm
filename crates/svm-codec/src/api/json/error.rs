#[derive(Debug, PartialEq, Eq)]
pub enum JsonError {
    InvalidJson(String),
    InvalidField { field: String, reason: String },
}
