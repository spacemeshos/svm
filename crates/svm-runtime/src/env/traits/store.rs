use crate::env::{self, hash};

use env::{ExtApp, ExtTemplate};
use hash::TemplateHash;

use svm_types::{AppAddr, AuthorAddr, SpawnerAddr, TemplateAddr};

/// A persistent store for `Template`(s).
pub trait TemplateStore {
    /// Stores template.
    ///
    /// template - Struct holding the data of Template (plus additional data such as the `author`)
    /// hash     - Template's code Hash.
    fn store(&mut self, template: &ExtTemplate, addr: &TemplateAddr, hash: &TemplateHash);

    /// Given a `Template` account address, fetches its raw data
    /// and deserializes it into `Template`. Returns `None` if `Template` doesn't exist.
    #[must_use]
    fn load(&self, addr: &TemplateAddr) -> Option<ExtTemplate>;
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
    fn find_template_addr(&self, addr: &AppAddr) -> Option<TemplateAddr>;
}
