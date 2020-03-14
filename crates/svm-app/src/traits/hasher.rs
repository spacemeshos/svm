use crate::types::{AppTemplate, AppTemplateHash};

/// Computes Hash derived deterministically from raw `AppTemplate`.
pub trait AppTemplateHasher {
    /// Given code as bytes, derives an Hash
    fn hash(template: &AppTemplate) -> AppTemplateHash;
}
