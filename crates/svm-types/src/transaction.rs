use std::fmt;

use crate::AppAddr;

/// An in-memory representation of an exec-app transaction.
#[derive(PartialEq)]
pub struct Transaction {
    /// The app-transaction version.
    pub version: u16,

    /// The `App` account address
    pub app: AppAddr,

    /// Function's name to execute
    pub func_name: String,

    /// Transaction's `VerifyData`
    pub verifydata: Vec<u8>,

    /// Transaction's `CallData`
    pub calldata: Vec<u8>,
}

impl Transaction {
    #[doc(hidden)]
    pub fn app_addr(&self) -> &AppAddr {
        &self.app
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
            .field("app", self.app.inner())
            .field("verifydata", &verifydata)
            .field("calldata", &calldata)
            .field("function", &self.func_name)
            .finish()
    }
}
