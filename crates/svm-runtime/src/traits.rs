use crate::{
    error::{DeployTemplateError, ExecAppError, SpawnAppError},
    host_ctx::HostCtx,
    settings::AppSettings,
    Receipt,
};

use svm_app::types::AppTransaction;
use svm_common::{Address, State};
use svm_storage::AppStorage;

/// Specifies the interface of a `SVM` Runtime.
pub trait Runtime {
    /// Deploy an new app-template
    fn deploy_template(
        &mut self,
        author: &Address,
        host_ctx: HostCtx,
        bytes: &[u8],
    ) -> Result<Address, DeployTemplateError>;

    /// Spawn a new app out of an existing app-template.
    fn spawn_app(
        &mut self,
        creator: &Address,
        host_ctx: HostCtx,
        bytes: &[u8],
    ) -> Result<(Address, State), SpawnAppError>;

    /// Parses `bytes` into in-memory `AppTransaction`
    fn parse_exec_app(
        &self,
        sender: &Address,
        bytes: &[u8],
    ) -> Result<AppTransaction, ExecAppError>;

    /// Executes an app-transaction. Returns a `Receipt`.
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
    ) -> Result<Receipt, ExecAppError>;
}

/// Represents a function that builds a `AppStorage` given its address, state and settings.
pub type StorageBuilderFn = dyn Fn(&Address, &State, &AppSettings) -> AppStorage;
