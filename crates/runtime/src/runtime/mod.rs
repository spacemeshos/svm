//! Implements the most high-level API of `SVM`.

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

use svm_types::{
    CallReceipt, DeployReceipt, DeployerAddr, Gas, RuntimeError, SpawnReceipt, SpawnerAddr, State,
    Transaction,
};

use crate::error::ValidateError;

/// Specifies the interface of a SVM [`Runtime`].
///
/// Any [`Runtime`] implementation will implement:
///
/// * `Deploy Template`s
/// * `Spawn `Account`s
/// * `Call Account`s
pub trait Runtime {
    /// Validates a raw `Deploy Template` transaction prior to executing it.
    fn validate_deploy(&self, bytes: &[u8]) -> Result<(), ValidateError>;

    /// Validates a raw `Spawn Account` transaction prior to executing it.
    fn validate_spawn(&self, bytes: &[u8]) -> Result<(), ValidateError>;

    /// Validates a raw `Call Account` transaction (a.k.a a [`Transaction`]) prior to executing it.
    fn validate_call(&self, bytes: &[u8]) -> Result<Transaction, ValidateError>;

    /// Deploy a `Template`
    fn deploy(&mut self, bytes: &[u8], deployer: &DeployerAddr, gas_limit: Gas) -> DeployReceipt;

    /// Spawns a new `Account`
    fn spawn(&mut self, bytes: &[u8], spawner: &SpawnerAddr, gas_limit: Gas) -> SpawnReceipt;

    /// Verifies a [`Transaction`] before execution.
    fn verify(&self, tx: &Transaction, state: &State, gas_limit: Gas)
        -> Result<bool, RuntimeError>;

    /// Executes an [`Transaction`] and returns its output [`CallReceipt`].
    ///
    /// This function should be called only if the `verify` stage passed.
    ///
    /// On Success:
    /// * Persists changes to the called `Account`'s storage.
    /// * Receipt returns the `Account`'s new `State`.
    /// * Receipt returns the amount of [`Gas`] used.
    ///
    /// On failure:
    /// * Receipt returns the occurred error
    /// * Receipt informs the amount of gas used (transaction gas limit)
    fn call(&self, tx: &Transaction, state: &State, gas_limit: Gas) -> CallReceipt;
}
