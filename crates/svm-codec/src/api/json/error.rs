#[derive(Debug, PartialEq, Eq)]
pub enum JsonError {
    Unknown(String),
    InvalidJson(String),
    InvalidField { field: String, reason: String },
}
