use crate::{
    error::StoreError,
    traits::{AppDeserializer, AppSerializer},
    types::{App, AppTemplate, AppTemplateHash},
};

use svm_common::Address;

/// A persistent store for `AppTemplate`(s)
pub trait AppTemplateStore {
    /// Stores the `Hash` -> `AppTemplate` and `Address -> Hash` relations.
    #[must_use]
    fn store(
        &mut self,
        template: &AppTemplate,
        address: &Address,
        hash: &AppTemplateHash,
    ) -> Result<(), StoreError>;

    /// Given a `AppTemplate` account address, fetches its raw data
    /// and deserializes it. Returns `None` if `AppTemplatee` doesn't exist.
    #[must_use]
    fn load(&self, template_addr: &Address) -> Option<AppTemplate>;
}

/// A persistent store for `App`(s)
pub trait AppStore {
    #[must_use]
    fn store(&mut self, app: &App, app_addr: &Address) -> Result<(), StoreError>;

    #[must_use]
    fn load(&self, app_addr: &Address) -> Option<App>;
}
