use crate::{Address, ReceiptLog, TemplateAddr};

/// Type for failed running transactions
#[doc(hidden)]
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
#[error("A runtime error happened.")]
pub enum RuntimeError {
    OOG,
    TemplateNotFound(TemplateAddr),
    AccountNotFound(Address),
    CompilationFailed {
        target: Address,
        template: TemplateAddr,
        msg: String,
    },
    InstantiationFailed {
        target: Address,
        template: TemplateAddr,
        msg: String,
    },
    FuncNotFound {
        target: Address,
        template: TemplateAddr,
        func: String,
    },
    FuncFailed {
        target: Address,
        template: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncNotAllowed {
        target: Address,
        template: TemplateAddr,
        func: String,
        msg: String,
    },
    FuncInvalidSignature {
        target: Address,
        template: TemplateAddr,
        func: String,
    },
}

/// A [`RuntimeError`] with some associated logs.
#[derive(Debug, PartialEq, Clone, thiserror::Error)]
#[error("Runtime error {:?} with the following logs: {:?}.", err, logs)]
pub struct RuntimeFailure {
    /// The [`RuntimeError`] with some logs.
    pub err: RuntimeError,
    /// The logs associated with `self.err`.
    pub logs: Vec<ReceiptLog>,
}

impl RuntimeFailure {
    /// Creates a new [`RuntimeFailure`].
    pub fn new(err: RuntimeError, logs: impl Into<Vec<ReceiptLog>>) -> Self {
        Self {
            err,
            logs: logs.into(),
        }
    }

    /// Returns the logs of `self` and leaves an empty [`Vec`] in their place.
    pub fn take_logs(&mut self) -> Vec<ReceiptLog> {
        std::mem::take(&mut self.logs)
    }
}

impl From<RuntimeError> for RuntimeFailure {
    fn from(err: RuntimeError) -> Self {
        RuntimeFailure {
            err,
            logs: Vec::new(),
        }
    }
}
