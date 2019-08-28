use crate::wasm::WasmArgValue;
use svm_common::{Address, State};

pub struct Tx {
    pub contract: Address,
    pub sender: Address,
    pub func_name: String,
    pub func_args: Vec<WasmArgValue>,
}
