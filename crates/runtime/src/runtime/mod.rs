mod call;
mod config;
mod default;
mod failure;
mod function;
mod outcome;
mod ptr;

pub use call::Call;
pub use failure::Failure;
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

use svm_types::{
    DeployerAddr, ExecReceipt, Gas, RuntimeError, SpawnAppReceipt, SpawnerAddr, State,
    TemplateReceipt, Transaction,
};

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
        deployer: &DeployerAddr,
        gas_limit: Gas,
    ) -> TemplateReceipt;

    /// Spawns a new app out of an existing app-template.
    fn spawn_app(&mut self, bytes: &[u8], spawner: &SpawnerAddr, gas_limit: Gas)
        -> SpawnAppReceipt;

    fn exec_verify(
        &self,
        tx: &Transaction,
        state: &State,
        gas_limit: Gas,
    ) -> Result<bool, RuntimeError>;

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
    fn exec_tx(&self, tx: &Transaction, state: &State, gas_limit: Gas) -> ExecReceipt;
}
