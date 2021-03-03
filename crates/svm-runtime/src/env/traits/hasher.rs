use crate::env::hash::TemplateHash;

use svm_types::Template;
/// Computes Hash derived deterministically from raw `Template`.
pub trait TemplateHasher {
    /// Given code as bytes, derives an Hash
    fn hash(template: &Template) -> TemplateHash;
}
