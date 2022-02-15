use std::fmt;

mod context;
mod envelope;
mod id;
mod layer;

pub use context::Context;
pub use envelope::Envelope;
pub use id::TransactionId;
pub use layer::Layer;

use crate::Address;

/// An in-memory representation of an `Call Account` transaction.
#[derive(PartialEq, Clone)]
pub struct Transaction {
    /// The `version`.
    pub version: u16,

    /// The target `Address`.
    pub target: Address,

    /// Function's name to execute
    pub func_name: String,

    /// Transaction's `VerifyData`
    pub verifydata: Vec<u8>,

    /// Transaction's `CallData`
    pub calldata: Vec<u8>,
}

impl Transaction {
    #[doc(hidden)]
    pub fn target(&self) -> &Address {
        &self.target
    }

    #[doc(hidden)]
    pub fn func_name(&self) -> &str {
        &self.func_name
    }

    #[doc(hidden)]
    pub fn verifydata(&self) -> &[u8] {
        &self.verifydata
    }

    #[doc(hidden)]
    pub fn calldata(&self) -> &[u8] {
        &self.calldata
    }
}

impl fmt::Debug for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let verifydata = self.verifydata.iter().take(4).collect::<Vec<_>>();
        let calldata = self.calldata.iter().take(4).collect::<Vec<_>>();

        f.debug_struct("Transaction")
            .field("version", &self.version)
            .field("target", self.target())
            .field("verifydata", &verifydata)
            .field("calldata", &calldata)
            .field("function", &self.func_name())
            .finish()
    }
}
