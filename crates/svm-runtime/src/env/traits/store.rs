use crate::env::types::TemplateHash;

use svm_types::{App, AppAddr, AuthorAddr, CreatorAddr, Template, TemplateAddr};

/// A persistent store for `Template`(s).
pub trait TemplateStore {
    /// Stores template.
    ///
    /// template - Struct holding the data of the Template. (struct representing the parsed raw data).
    /// author   - The `Address` of the Template Author.
    /// addr     - The `Address` of the Template.
    /// hash     - Template's code Hash.
    fn store(
        &mut self,
        template: &Template,
        author: &AuthorAddr,
        addr: &TemplateAddr,
        hash: &TemplateHash,
    );

    /// Given a `Template` account address, fetches its raw data
    /// and deserializes it into `Template`. Returns `None` if `Template` doesn't exist.
    #[must_use]
    fn load(&self, addr: &TemplateAddr) -> Option<(Template, AuthorAddr)>;
}

/// A persistent store for `A}pp`(s)
pub trait AppStore {
    /// Stores `Address` -> `App`
    fn store(&mut self, app: &App, creator: &CreatorAddr, addr: &AppAddr);

    /// Given a `App` account address, fetches its raw data
    /// and deserializes it into `App`. Returns `None` if `Template` doesn't exist.
    #[must_use]
    fn load(&self, addr: &AppAddr) -> Option<(App, CreatorAddr)>;
}
