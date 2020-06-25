use crate::{
    error::ValidateError,
    gas::MaybeGas,
    receipt::{ExecReceipt, SpawnAppReceipt, TemplateReceipt},
};

use svm_common::State;
use svm_gas::Gas;
use svm_types::{AppAddr, AuthorAddr, CreatorAddr, HostCtx};

/// Specifies the interface of a `SVM` Runtime.
pub trait Runtime {
    /// Validates raw `deploy-template` transaction prior to executing it.
    fn validate_template(&self, bytes: &[u8]) -> Result<(), ValidateError>;

    /// Validates a raw `spawn-app` transaction prior to executing it.
    fn validate_app(&self, bytes: &[u8]) -> Result<(), ValidateError>;

    /// Validates a raw `exec-app` transaction prior to executing it.
    fn validate_tx(&self, bytes: &[u8]) -> Result<AppAddr, ValidateError>;

    /// Estimates the `Gas` required for deploying template givee as raw `bytes`.
    fn estimate_deploy_template(&self, bytes: &[u8]) -> Result<Gas, ValidateError>;

    /// Estimates the `Gas` required for spawning app given as raw `bytes`.
    fn estimate_spawn_app(&self, bytes: &[u8]) -> Result<Gas, ValidateError>;

    /// Estimates the `Gas` required for executing app-transaction given as raw `bytes`.
    fn estimate_exec_app(&self, bytes: &[u8]) -> Result<Gas, ValidateError>;

    /// Deploy an new app-template
    fn deploy_template(
        &mut self,
        bytes: &[u8],
        author: &AuthorAddr,
        host_ctx: HostCtx,
        gas_limit: MaybeGas,
    ) -> TemplateReceipt;

    /// Spawn a new app out of an existing app-template.
    fn spawn_app(
        &mut self,
        bytes: &[u8],
        creator: &CreatorAddr,
        host_ctx: HostCtx,
        gas_limit: MaybeGas,
    ) -> SpawnAppReceipt;

    /// Executes an app-transaction. Returns `ExecReceipt`.
    /// On success:
    /// * Persists changes to the app's own storage.
    /// * Receipt returns the app's new storage state.
    /// * Receipt informs the amount of gas used.
    ///
    /// On failure:
    /// * Receipt returns the occurred error
    /// * Receipt informs the amount of gas used (transaction gas limit)
    fn exec_app(
        &self,
        bytes: &[u8],
        state: &State,
        host_ctx: HostCtx,
        gas_limit: MaybeGas,
    ) -> ExecReceipt;
}
