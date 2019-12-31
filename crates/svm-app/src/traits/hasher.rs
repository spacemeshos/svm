use crate::types::AppTemplateHash;

/// Computes Hash derived deterministically from raw `AppTemplate`.
pub trait AppTemplateHasher {
    /// Given code as bytes, derives an Hash
    fn hash(bytes: &[u8]) -> AppTemplateHash;
}
