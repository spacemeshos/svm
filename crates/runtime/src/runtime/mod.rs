//! Implements the most high-level API of `SVM`.

mod call;
mod config;
mod default;
mod failure;
mod function;
mod outcome;

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

use svm_types::{CallReceipt, Context, DeployReceipt, Envelope, SpawnReceipt};

use crate::error::ValidateError;

/// Specifies the interface of a SVM [`Runtime`].
///
/// Any [`Runtime`] implementation will implement:
///
/// * `Deploy Template`s
/// * `Spawn Account`s
/// * `Call Account`s
pub trait Runtime {
    /// Validates syntactically a binary `Deploy Template` message prior to executing it.
    fn validate_deploy(&self, message: &[u8]) -> Result<(), ValidateError>;

    /// Validates syntactically a binary `Spawn Account` message prior to executing it.
    fn validate_spawn(&self, message: &[u8]) -> Result<(), ValidateError>;

    /// Validates syntactically a binary `Call Account` message prior to executing it.
    fn validate_call(&self, message: &[u8]) -> Result<(), ValidateError>;

    /// Deploys a `Template`
    fn deploy(&mut self, envelope: &Envelope, message: &[u8], context: &Context) -> DeployReceipt;

    /// Spawns a new `Account`
    fn spawn(&mut self, envelope: &Envelope, message: &[u8], context: &Context) -> SpawnReceipt;

    /// Verifies a [`Transaction`](svm_types::Transaction) before execution.
    fn verify(&mut self, envelope: &Envelope, message: &[u8], context: &Context) -> CallReceipt;

    /// Executes a [`Transaction`](svm_types::Transaction) and returns its output [`CallReceipt`].
    ///
    /// This function should be called only if the `verify` stage has passed.
    fn call(&mut self, envelope: &Envelope, message: &[u8], context: &Context) -> CallReceipt;
}
