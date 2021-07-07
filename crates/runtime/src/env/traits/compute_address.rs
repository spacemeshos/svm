use svm_types::{AppAddr, Template, TemplateAddr};

use crate::env::ExtSpawnApp;

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
    fn compute(app: &ExtSpawnApp) -> AppAddr;
}
