use svm_common::Address;

/// Contract execution error
#[allow(missing_docs)]
pub enum ContractExecError {
    NotFound(Address),
    CompilationFailed(Address),
    InstantiationFailed(Address),
    FuncNotFound(String),
    ExecFailed,
}

impl std::error::Error for ContractExecError {
    fn description(&self) -> &'static str {
        match self {
            ContractExecError::NotFound(_) => "Contract not found",
            ContractExecError::CompilationFailed(_) => "Compilation failed",
            ContractExecError::InstantiationFailed(_) => "Instance Instantiation failed",
            ContractExecError::FuncNotFound(_) => "Function not found",
            ContractExecError::ExecFailed => "Execution failed",
        }
    }
}

impl std::fmt::Display for ContractExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            ContractExecError::NotFound(addr) => format!("Contract `{:?}` not found", addr),
            ContractExecError::CompilationFailed(addr) => {
                format!("Compilation failed for contract `{:?}`", addr)
            }
            ContractExecError::InstantiationFailed(addr) => {
                format!("Instance Instantiation failed for contract `{:?}`", addr)
            }
            ContractExecError::FuncNotFound(func) => format!("Function `{}` not found", func),
            ContractExecError::ExecFailed => "Execution failed".to_string(),
        };

        write!(f, "{}", msg)
    }
}

impl std::fmt::Debug for ContractExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }
}
