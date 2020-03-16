use crate::{
    error::{DeployTemplateError, ExecAppError, SpawnAppError},
    receipt::{ExecReceipt, SpawnAppReceipt, TemplateReceipt},
    settings::AppSettings,
};

use svm_app::{
    error::ParseError,
    types::{
        AppAddr, AppTemplate, AppTransaction, AuthorAddr, CreatorAddr, HostCtx, SpawnApp,
        TemplateAddr,
    },
};

use svm_common::{Address, State};
use svm_storage::AppStorage;

/// Specifies the interface of a `SVM` Runtime.
pub trait Runtime {
    /// Validates raw `deploy-template` transaction prior to executing it.
    fn vaildate_template(&self, bytes: &[u8]) -> Result<(), ParseError>;

    /// Validates a raw `spawn-app` transaction prior to executing it.
    fn vaildate_app(&self, bytes: &[u8]) -> Result<(), ParseError>;

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
        app_tx: AppTransaction,
        state: State,
        host_ctx: HostCtx,
        dry_run: bool,
    ) -> ExecReceipt;
}

pub trait GasEstimator {
    fn estimate_template(bytes: &[u8]) -> u64;

    fn estimate_app(bytes: &[u8]) -> u64;

    fn estimate_tx(bytes: &[u8]) -> u64;
}

/// Represents a function that builds a `AppStorage` given its address, state and settings.
pub type StorageBuilderFn = dyn Fn(&AppAddr, &State, &AppSettings) -> AppStorage;
