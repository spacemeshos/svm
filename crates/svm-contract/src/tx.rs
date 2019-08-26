use svm_common::{Address, State};

pub struct Tx {
    pub contract: Address,
    pub sender: Address,
    pub state: State,
    pub func_name: String,
    pub func_args: Vec<u8>,
}
