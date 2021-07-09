use svm_hash::{DefaultHasher, Hasher};
use svm_types::Template;

use super::super::traits::TemplateHasher;
use crate::env::{traits, TemplateHash};

/// Default implementation for `TemplateCodeHasher`
pub struct DefaultTemplateHasher;

impl TemplateHasher for DefaultTemplateHasher {
    #[inline]
    fn hash(template: &Template) -> TemplateHash {
        DefaultHasher::hash(template.code())
    }
}
