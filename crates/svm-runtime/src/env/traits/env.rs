use crate::env::traits;

use traits::{AppAddressCompute, AppStore, TemplateAddressCompute, TemplateHasher, TemplateStore};

/// Aggregates types that are required by `Env`
pub trait EnvTypes {
    /// `Template` store type.
    type TemplateStore: TemplateStore;

    /// `AppStore` store type.
    type AppStore: AppStore;

    /// Compute `Template` address type.
    type TemplateAddressCompute: TemplateAddressCompute;

    /// Compute `App` address type.
    type AppAddressCompute: AppAddressCompute;

    /// `Template` content Hasher type.
    type TemplateHasher: TemplateHasher;
}
