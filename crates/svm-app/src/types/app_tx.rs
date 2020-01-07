use std::fmt;

use crate::types::WasmArgValue;

use svm_common::Address;

/// An in-memory representation of an `App` transaction.
#[derive(Clone, PartialEq)]
pub struct AppTransaction {
    /// The `App` account address
    pub app: Address,

    /// AppTransaction sender account address
    pub sender: Address,

    /// `App` function to execute
    pub func_name: String,

    /// `App` function args
    pub func_args: Vec<WasmArgValue>,
}

impl fmt::Debug for AppTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let app = self.fmt_app();
        let sender = self.fmt_sender();
        let func_name = self.fmt_func_name();
        let func_args = self.fmt_func_args();

        let msg = [app, sender, func_name, func_args];

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

    fn fmt_func_name(&self) -> String {
        format!("FuncName: {:?}", self.func_name)
    }

    fn fmt_func_arg(&self, func_arg: &WasmArgValue) -> String {
        format!("{:?}", func_arg)
    }

    fn fmt_func_args(&self) -> String {
        let mut args_str = Vec::with_capacity(self.func_args.len());

        for arg in self.func_args.iter() {
            let arg_str = self.fmt_func_arg(arg);
            args_str.push(arg_str);
        }

        format!("FuncArgs: {}", args_str.join(", "))
    }
}
