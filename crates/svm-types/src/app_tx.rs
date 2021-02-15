use std::fmt;

use crate::AppAddr;

/// An in-memory representation of an exec-app transaction.
#[derive(PartialEq)]
pub struct AppTransaction {
    /// The app-transaction version.
    pub version: u16,

    /// The `App` account address
    pub app: AppAddr,

    /// Function's name to execute
    pub func_name: String,

    /// Transaction's calldata
    pub calldata: Vec<u8>,
}

impl fmt::Debug for AppTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let calldata = self.calldata.iter().take(4).collect::<Vec<_>>();

        f.debug_struct("AppTransaction")
            .field("version", &self.version)
            .field("app", self.app.inner())
            .field("calldata", &calldata)
            .field("function", &self.func_name)
            .finish()
    }
}
