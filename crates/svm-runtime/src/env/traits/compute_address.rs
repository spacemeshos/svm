use svm_types::{AppAddr, Template, SpawnApp, TemplateAddr};

/// Computes an `AppTemplate` account address.
/// Algorithm must be deterministic.
pub trait AppTemplateAddressCompute {
    /// Derives the `AppTemplate` address
    fn compute(template: &Template) -> TemplateAddr;
}

/// Computes an `App` account address.
/// Algorithm must be deterministic.
pub trait AppAddressCompute {
    /// Derives the `App` address
    fn compute(app: &SpawnApp) -> AppAddr;
}
