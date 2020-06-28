use std::fmt;

use crate::{Address, AppAddr, WasmValue};

/// An in-memory representation of an exec-app transaction.
#[derive(PartialEq)]
pub struct AppTransaction {
    /// The app-transaction version.
    pub version: u32,

    /// The `App` account address
    pub app: AppAddr,

    /// Function Export Index to execute
    pub func_idx: u16,

    /// Function buffer
    pub func_buf: Vec<u8>,

    /// `App` function args
    pub func_args: Vec<WasmValue>,
}

impl fmt::Debug for AppTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = self.fmt_version();
        let app = self.fmt_app();
        let func_idx = self.fmt_func_idx();
        let func_buf = self.fmt_func_buf();
        let func_args = self.fmt_func_args();

        let msg = [version, app, func_idx, func_buf, func_args];

        write!(f, "{}", msg.join("\n"))
    }
}

impl AppTransaction {
    fn fmt_version(&self) -> String {
        format!("Version: {}", self.version)
    }

    fn fmt_app(&self) -> String {
        todo!()
        // format!("App: {}", AppTransaction::fmt_address(&self.app.inner()))
    }

    fn fmt_func_idx(&self) -> String {
        format!("func_idx: {}", self.func_idx)
    }

    fn fmt_func_buf(&self) -> String {
        format!(
            "func_buf: {:?}",
            self.func_buf.iter().take(4).collect::<Vec<_>>()
        )
    }

    fn fmt_func_args(&self) -> String {
        format!("func_args: {:?}", self.func_args)
    }

    fn fmt_address(addr: &Address) -> String {
        addr.fmt(4, 4, " ")
    }
}
