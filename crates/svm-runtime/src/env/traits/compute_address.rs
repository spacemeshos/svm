use svm_types::{AppAddr, TemplateAddr};

use crate::env::{ExtSpawnApp, ExtTemplate};

/// Computes an `Template` account address.
/// Algorithm must be deterministic.
pub trait TemplateAddressCompute {
    /// Derives the `Template` address
    fn compute(template: &ExtTemplate) -> TemplateAddr;
}

/// Computes an `App` account address.
/// Algorithm must be deterministic.
pub trait AppAddressCompute {
    /// Derives the `App` address
    fn compute(app: &ExtSpawnApp) -> AppAddr;
}
