use crate::types::{AppTemplateHash, DeployAppTemplate};

/// Computes Hash derived deterministically from raw `AppTemplate`.
pub trait AppTemplateHasher {
    /// Given code as bytes, derives an Hash
    fn hash(template: &DeployAppTemplate) -> AppTemplateHash;
}
