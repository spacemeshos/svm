use crate::{
    error::StoreError,
    types::{App, AppAddr, AppTemplate, AppTemplateHash, AuthorAddr, CreatorAddr, TemplateAddr},
};

/// A persistent store for `AppTemplate`(s).
pub trait AppTemplateStore {
    /// Stores template.
    ///
    /// template - Struct holding the data of the Template. (struct representing the parsed raw data).
    /// author   - The `Address` of the Template Author.
    /// addr     - The `Address` of the Template.
    /// hash     - Template's code Hash.
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
