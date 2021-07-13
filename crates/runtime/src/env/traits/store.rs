use std::collections::HashSet;

use crate::env::{self, hash};

use env::ExtApp;
use hash::TemplateHash;

use svm_types::{AppAddr, SectionKind, Template, TemplateAddr};

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

/// A persistent store for `A}pp`(s)
pub trait AppStore {
    /// Stores `Address` -> `App`
    fn store(&mut self, app: &ExtApp, addr: &AppAddr);

    /// Given a `App` account `Address`, fetches its raw data
    /// and deserializes it into `App`. Returns `None` if `Template` doesn't exist.
    #[must_use]
    fn load(&self, addr: &AppAddr) -> Option<ExtApp>;

    #[must_use]
    fn resolve_template_addr(&self, addr: &AppAddr) -> Option<TemplateAddr>;
}
