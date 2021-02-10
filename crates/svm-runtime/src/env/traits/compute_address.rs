use svm_types::{AppAddr, SpawnApp, Template, TemplateAddr};

/// Computes an `Template` account address.
/// Algorithm must be deterministic.
pub trait TemplateAddressCompute {
    /// Derives the `Template` address
    fn compute(template: &Template) -> TemplateAddr;
}

/// Computes an `App` account address.
/// Algorithm must be deterministic.
pub trait AppAddressCompute {
    /// Derives the `App` address
    fn compute(app: &SpawnApp) -> AppAddr;
}
