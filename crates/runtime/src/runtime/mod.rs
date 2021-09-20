//! Implements the most high-level API of `SVM`.

mod call;
mod default;
mod function;
mod outcome;

use svm_types::{
    Address, CallReceipt, Context, DeployReceipt, Envelope, Layer, RuntimeFailure, SpawnReceipt,
    TemplateAddr,
};

use crate::error::ValidateError;

pub use call::Call;
pub use default::DefaultRuntime;
pub use function::Function;
pub use outcome::Outcome;

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

    /// Moves the internal state of this [`Runtime`] back to the time of
    /// `layer_id`.
    fn rewind(&mut self, layer_id: Layer) -> Result<(), RuntimeFailure>;

    /// Creates a new layer with the given changes.
    fn commit(&mut self) -> Result<(), RuntimeFailure>;

    /// Given the address of an account, it attempts to read:
    ///
    /// - balance;
    /// - counter;
    /// - template's address;
    ///
    /// from the database layer.
    fn get_account(&self, account_addr: &Address) -> Option<(u64, u128, TemplateAddr)>;
}
