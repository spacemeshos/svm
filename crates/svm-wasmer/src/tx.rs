use svm_common::{Address, State};

pub struct Tx {
    pub Contract: Address,
    pub Sender: Address,
    pub State: State,
    pub FuncName: String,
    pub FuncArgs: Vec<u8>,
}
