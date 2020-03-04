use crate::types::{App, AppTemplate, HostCtx};

use svm_common::Address;

/// Computes an `AppTemplate` account address.
/// Algorithm must be deterministic.
pub trait AppTemplateAddressCompute {
    /// Derives the `AppTemplate` address
    fn compute(template: &AppTemplate, host_ctx: &HostCtx) -> Address;
}

/// Computes an `App` account address.
/// Algorithm must be deterministic.
pub trait AppAddressCompute {
    /// Derives the `App` address
    fn compute(app: &App, host_ctx: &HostCtx) -> Address;
}
