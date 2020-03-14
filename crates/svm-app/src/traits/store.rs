use crate::{
    error::StoreError,
    types::{App, AppAddr, AppTemplate, AppTemplateHash, AuthorAddr, CreatorAddr, TemplateAddr},
};

use svm_common::Address;

/// A persistent store for `AppTemplate`(s)
pub trait AppTemplateStore {
    #[must_use]
    fn store(
        &mut self,
        template: &AppTemplate,
        author: &AuthorAddr,
        addr: &TemplateAddr,
        hash: &AppTemplateHash,
    ) -> Result<(), StoreError>;

    /// Given a `AppTemplate` account address, fetches its raw data
    /// and deserializes it into `AppTemplate`. Returns `None` if `AppTemplatee` doesn't exist.
    #[must_use]
    fn load(&self, addr: &TemplateAddr) -> Option<(AppTemplate, AuthorAddr)>;
}

/// A persistent store for `A}pp`(s)
pub trait AppStore {
    /// Stores `Address` -> `App`
    #[must_use]
    fn store(&mut self, app: &App, creator: &CreatorAddr, addr: &AppAddr)
        -> Result<(), StoreError>;

    /// Given a `App` account address, fetches its raw data
    /// and deserializes it into `App`. Returns `None` if `AppTemplate` doesn't exist.
    #[must_use]
    fn load(&self, addr: &AppAddr) -> Option<(App, CreatorAddr)>;
}
