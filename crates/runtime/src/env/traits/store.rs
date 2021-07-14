use svm_types::{AccountAddr, SectionKind, Template, TemplateAddr};

use std::collections::HashSet;

use crate::env::{ExtApp, TemplateHash};

/// A persistent store for `Template`(s).
pub trait TemplateStore {
    /// Stores template.
    ///
    /// `template` - Struct holding the data of a `Template`
    /// `hash`     - Template's code Hash.
    fn store(&mut self, template: &Template, addr: &TemplateAddr, hash: &TemplateHash);

    /// Given a `Template` account `Address`, fetches its raw data and deserializes it into `Template`.
    /// Returns `None` if `Template` doesn't exist.
    #[must_use]
    fn load(
        &self,
        addr: &TemplateAddr,
        interests: Option<HashSet<SectionKind>>,
    ) -> Option<Template>;
}

/// A persistent store for `App`(s)
pub trait AppStore {
    /// Stores `Address` -> `App`
    fn store(&mut self, app: &ExtApp, addr: &AccountAddr);

    /// Given a `App` account `Address`, fetches its raw data
    /// and deserializes it into `App`. Returns `None` if `Template` doesn't exist.
    #[must_use]
    fn load(&self, addr: &AccountAddr) -> Option<ExtApp>;

    #[must_use]
    fn resolve_template_addr(&self, addr: &AccountAddr) -> Option<TemplateAddr>;
}
