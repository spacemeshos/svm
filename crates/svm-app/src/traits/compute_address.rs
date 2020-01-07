use crate::types::{App, AppTemplate};

use svm_common::Address;

/// Computes an `AppTemplate` account address.
/// Algorithm must be deterministic.
pub trait AppTemplateAddressCompute {
    /// Derives the `AppTemplate` address
    fn compute(template: &AppTemplate) -> Address;
}

pub trait AppAddressCompute {
    /// Derives the `AppAddress` address
    fn compute(app: &App) -> Address;
}
