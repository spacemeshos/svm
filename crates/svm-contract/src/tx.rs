use crate::wasm::WasmArgValue;
use svm_common::{Address, State};

#[derive(Clone, PartialEq)]
pub struct Tx {
    pub contract: Address,
    pub sender: Address,
    pub func_name: String,
    pub func_args: Vec<WasmArgValue>,
}

impl std::fmt::Debug for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let contract = self.fmt_contract();
        let sender = self.fmt_sender();
        let func_name = self.fmt_func_name();
        let func_args = self.fmt_func_args();

        let msg = [contract, sender, func_name, func_args];

        write!(f, "{}", msg.join("\n"))
    }
}

impl Tx {
    fn fmt_contract(&self) -> String {
        self.fmt_address("Contract", self.contract)
    }

    fn fmt_sender(&self) -> String {
        self.fmt_address("Sender", self.sender)
    }

    fn fmt_address(&self, field: &str, addr: Address) -> String {
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
