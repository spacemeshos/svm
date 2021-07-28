use svm_types::{ReceiptLog, RuntimeError};

#[derive(Debug, PartialEq, Clone)]
pub struct Failure {
    err: RuntimeError,
    logs: Vec<ReceiptLog>,
}

impl Failure {
    pub fn new(err: RuntimeError, logs: Vec<ReceiptLog>) -> Self {
        Self { err, logs }
    }

    pub fn take_logs(&mut self) -> Vec<ReceiptLog> {
        std::mem::take(&mut self.logs)
    }

    pub fn take_error(self) -> RuntimeError {
        self.err
    }
}

impl From<RuntimeError> for Failure {
    fn from(err: RuntimeError) -> Self {
        Failure::new(err, Vec::new())
    }
}
