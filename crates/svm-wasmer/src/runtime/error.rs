use svm_common::Address;

pub enum ContractExecError {
    NotFound(Address),
    CompilationFailed(Address),
    InstantiationFailed(Address),
    FuncNotFound(String),
    ExecFailed,
}
