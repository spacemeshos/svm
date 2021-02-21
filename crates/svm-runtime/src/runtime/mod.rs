mod call;
mod config;
mod default;
mod function;
mod outcome;
mod ptr;

pub use call::{Call, CallAddr, CallKind};
pub use function::Function;
pub use outcome::Outcome;

#[cfg(feature = "default-rocksdb")]
mod rocksdb;

#[cfg(feature = "default-rocksdb")]
pub use rocksdb::create_rocksdb_runtime;

pub use config::Config;
pub use default::DefaultRuntime;
pub use ptr::RuntimePtr;

use crate::error::ValidateError;

use svm_types::receipt::{ExecReceipt, SpawnAppReceipt, TemplateReceipt};
use svm_types::{gas::MaybeGas, AppAddr, AuthorAddr, SpawnerAddr, State};
use svm_types::{RuntimeError, Transaction};

/// Specifies the interface of a `SVM` Runtime.
pub trait Runtime {
    /// Validates raw `deploy-template` transaction prior to executing it.
    fn validate_template(&self, bytes: &[u8]) -> Result<(), ValidateError>;

    /// Validates a raw `spawn-app` transaction prior to executing it.
    fn validate_app(&self, bytes: &[u8]) -> Result<(), ValidateError>;

    /// Validates a raw `exec-app` transaction prior to executing it.
    fn validate_tx(&self, bytes: &[u8]) -> Result<Transaction, ValidateError>;

    /// Deploy an new app-template
    fn deploy_template(
        &mut self,
        bytes: &[u8],
        author: &AuthorAddr,
        gas_limit: MaybeGas,
    ) -> TemplateReceipt;

    /// Spawns a new app out of an existing app-template.
    fn spawn_app(
        &mut self,
        bytes: &[u8],
        spawner: &SpawnerAddr,
        gas_limit: MaybeGas,
    ) -> SpawnAppReceipt;

    /// Executes an transaction. Returns `ExecReceipt`.
    /// Should be called only if the `verify` stage passed.
    ///
    /// On success:
    /// * Persists changes to the app's own storage.
    /// * Receipt returns the app's new storage state.
    /// * Receipt informs the amount of gas used.
    ///
    /// On failure:
    /// * Receipt returns the occurred error
    /// * Receipt informs the amount of gas used (transaction gas limit)
    fn exec_app(&self, tx: &Transaction, state: &State, gas_limit: MaybeGas) -> ExecReceipt;
}
