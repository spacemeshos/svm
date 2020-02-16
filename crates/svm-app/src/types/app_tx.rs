use std::fmt;

use crate::types::WasmValue;

use svm_common::Address;

/// An in-memory representation of an exec-app transaction.
#[derive(Clone, PartialEq)]
pub struct AppTransaction {
    /// The `App` account address
    pub app: Address,

    /// Sender account address
    pub sender: Address,

    /// Function buffer
    pub func_buf: Vec<u8>,

    /// Function Export Index to execute
    pub func_idx: u16,

    /// `App` function args
    pub func_args: Vec<WasmValue>,
}

impl fmt::Debug for AppTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let app = self.fmt_app();
        let sender = self.fmt_sender();
        let func_idx = self.fmt_func_index();
        let func_args = self.fmt_func_args();
        let func_buf = self.fmt_func_buf();

        let msg = [app, sender, func_idx, func_args, func_buf];

        write!(f, "{}", msg.join("\n"))
    }
}

impl AppTransaction {
    fn fmt_app(&self) -> String {
        self.fmt_address("App", &self.app)
    }

    fn fmt_sender(&self) -> String {
        self.fmt_address("Sender", &self.sender)
    }

    fn fmt_address(&self, field: &str, addr: &Address) -> String {
        format!("{:?}: {:?}", field, addr)
    }

    fn fmt_func_index(&self) -> String {
        format!("Func index: {:?}", self.func_idx)
    }

    fn fmt_func_arg(&self, func_arg: &WasmValue) -> String {
        format!("{:?}", func_arg)
    }

    fn fmt_func_buf(&self) -> String {
        // todo!()
        "...".to_string()
    }

    fn fmt_func_args(&self) -> String {
        let mut args_str = Vec::with_capacity(self.func_args.len());

        for arg in self.func_args.iter() {
            let arg_str = self.fmt_func_arg(arg);
            args_str.push(arg_str);
        }

        format!("Func args: {}", args_str.join(", "))
    }
}
