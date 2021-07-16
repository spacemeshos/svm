use std::collections::HashSet;

use svm_types::{AccountAddr, SectionKind, Template, TemplateAddr};

use crate::env::{ExtAccount, TemplateHash};

/// A persistent store for [`Template`](s).
pub trait TemplateStore {
    /// Stores a [`Template`].
    ///
    /// parameters:
    ///
    /// `template` - Struct holding the data of the [`Template`].
    /// `addr`     - [`Template`]'s `Address`.
    /// `hash`     - [`Template`]'s [`TemplateHash`].
    fn store(&mut self, template: &Template, addr: &TemplateAddr, hash: &TemplateHash);

    /// Given a [`Template`]'s `Address`, fetches its raw data and deserializes it into `Template`.
    /// Returns `None` if [`Template`] doesn't exist.
    #[must_use]
    fn load(
        &self,
        addr: &TemplateAddr,
        interests: Option<HashSet<SectionKind>>,
    ) -> Option<Template>;
}

/// A persistent store for `Account`(s)
pub trait AccountStore {
    /// Stores `Address` -> `Account`
    fn store(&mut self, account: &ExtAccount, addr: &AccountAddr);

    /// Given a `Account` `Address`, fetches its raw data
    /// and deserializes it into an [`ExtAccount`].
    ///
    /// Returns `None` if [`Template`] doesn't exist.
    #[must_use]
    fn load(&self, addr: &AccountAddr) -> Option<ExtAccount>;

    #[must_use]
    fn resolve_template_addr(&self, addr: &AccountAddr) -> Option<TemplateAddr>;
}
