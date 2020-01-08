use std::error;
use std::fmt;

use svm_app::error::ParseError;
use svm_common::Address;

/// `AppTransaction` execution error
#[allow(missing_docs)]
#[derive(PartialEq, Clone)]
pub enum ExecAppError {
    ParseFailed(ParseError),
    AppNotFound(Address),
    CompilationFailed(Address, String),
    InstantiationFailed(Address, String),
    FuncNotFound(String),
    ExecFailed(String),
    InvalidResultValue(String),
}

impl error::Error for ExecAppError {
    fn description(&self) -> &'static str {
        match self {
            ExecAppError::ParseFailed(..) => "Parse failed",
            ExecAppError::AppNotFound(..) => "App not found",
            ExecAppError::CompilationFailed(..) => "Compilation failed",
            ExecAppError::InstantiationFailed(..) => "Instance Instantiation failed",
            ExecAppError::FuncNotFound(..) => "Function not found",
            ExecAppError::ExecFailed(..) => "Execution failed",
            ExecAppError::InvalidResultValue(..) => "Invalid result value",
        }
    }
}

impl fmt::Display for ExecAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ExecAppError::ParseFailed(e) => format!("{:?}", e),
            ExecAppError::AppNotFound(addr) => format!("App `{:?}` not found", addr),
            ExecAppError::CompilationFailed(addr, e) => {
                format!("Compilation failed for template `{:?}` ({})", addr, e)
            }
            ExecAppError::InstantiationFailed(addr, e) => format!(
                "Instance Instantiation failed for template `{:?}` ({})",
                addr, e
            ),
            ExecAppError::FuncNotFound(func) => format!("Function `{}` not found", func),
            ExecAppError::InvalidResultValue(val) => format!("Invalid result value: `{}`", val),
            ExecAppError::ExecFailed(e) => format!("Execution failed ({})", e),
        };

        write!(f, "{}", msg)
    }
}

impl fmt::Debug for ExecAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}
