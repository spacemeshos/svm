use crate::env::types::AppTemplateHash;
use svm_types::Template;

/// Computes Hash derived deterministically from raw `AppTemplate`.
pub trait AppTemplateHasher {
    /// Given code as bytes, derives an Hash
    fn hash(template: &Template) -> AppTemplateHash;
}
