use crate::types::{DeployAppTemplate, HostCtx, SpawnApp};

use svm_common::Address;

/// Computes an `AppTemplate` account address.
/// Algorithm must be deterministic.
pub trait AppTemplateAddressCompute {
    /// Derives the `AppTemplate` address
    fn compute(template: &DeployAppTemplate, host_ctx: &HostCtx) -> Address;
}

/// Computes an `App` account address.
/// Algorithm must be deterministic.
pub trait AppAddressCompute {
    /// Derives the `App` address
    fn compute(app: &SpawnApp, host_ctx: &HostCtx) -> Address;
}
