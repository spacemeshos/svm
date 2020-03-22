use crate::receipt::{ExecReceipt, SpawnAppReceipt, TemplateReceipt};

use svm_app::{
    error::ParseError,
    types::{AppAddr, AuthorAddr, CreatorAddr, HostCtx},
};
use svm_common::State;

/// Specifies the interface of a `SVM` Runtime.
pub trait Runtime {
    /// Validates raw `deploy-template` transaction prior to executing it.
    fn validate_template(&self, bytes: &[u8]) -> Result<(), ParseError>;

    /// Validates a raw `spawn-app` transaction prior to executing it.
    fn validate_app(&self, bytes: &[u8]) -> Result<(), ParseError>;

    /// Validates a raw `exec-app` transaction prior to executing it.
    fn validate_tx(&self, bytes: &[u8]) -> Result<AppAddr, ParseError>;

    /// Deploy an new app-template
    fn deploy_template(
        &mut self,
        bytes: &[u8],
        author: &AuthorAddr,
        host_ctx: HostCtx,
        dry_run: bool,
    ) -> TemplateReceipt;

    /// Spawn a new app out of an existing app-template.
    fn spawn_app(
        &mut self,
        bytes: &[u8],
        creator: &CreatorAddr,
        host_ctx: HostCtx,
        dry_run: bool,
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
        dry_run: bool,
    ) -> ExecReceipt;
}
