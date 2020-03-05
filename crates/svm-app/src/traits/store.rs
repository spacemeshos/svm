use crate::{
    error::StoreError,
    types::{App, AppTemplate, AppTemplateHash, DeployAppTemplate, SpawnApp},
};

use svm_common::Address;

/// A persistent store for `AppTemplate`(s)
pub trait AppTemplateStore {
    /// Stores the following:
    /// * `Hash` -> `AppTemplate`
    /// * `Address -> Hash` relations.
    #[must_use]
    fn store(
        &mut self,
        template: &DeployAppTemplate,
        address: &Address,
        hash: &AppTemplateHash,
    ) -> Result<(), StoreError>;

    /// Given a `AppTemplate` account address, fetches its raw data
    /// and deserializes it into `AppTemplate`. Returns `None` if `AppTemplatee` doesn't exist.
    #[must_use]
    fn load(&self, template_addr: &Address) -> Option<AppTemplate>;
}

/// A persistent store for `A}pp`(s)
pub trait AppStore {
    /// Stores `Address` -> `App`
    #[must_use]
    fn store(&mut self, app: &SpawnApp, app_addr: &Address) -> Result<(), StoreError>;

    /// Given a `App` account address, fetches its raw data
    /// and deserializes it into `App`. Returns `None` if `AppTemplatee` doesn't exist.
    #[must_use]
    fn load(&self, app_addr: &Address) -> Option<App>;
}
