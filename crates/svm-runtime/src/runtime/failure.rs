use svm_types::receipt::Log;
use svm_types::RuntimeError;

#[derive(Debug, PartialEq, Clone)]
pub struct Failure {
    err: RuntimeError,

    logs: Vec<Log>,
}

impl Failure {
    pub fn new(err: RuntimeError, logs: Vec<Log>) -> Self {
        Self { err, logs }
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }

    pub fn error(&self) -> &RuntimeError {
        &self.err
    }
}

impl From<RuntimeError> for Failure {
    fn from(err: RuntimeError) -> Self {
        Failure::new(err, Vec::new())
    }
}
