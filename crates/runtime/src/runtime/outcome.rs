use svm_types::{Gas, Log};

pub struct Outcome<T> {
    returns: T,

    gas_used: Gas,

    logs: Vec<Log>,
}

impl<T> Outcome<T> {
    pub fn map<S, F>(self, f: F) -> Outcome<S>
    where
        F: Fn(T) -> S,
    {
        Outcome::new(f(self.returns), self.gas_used, self.logs)
    }
}

impl<T> Outcome<T> {
    pub fn new(returns: T, gas_used: Gas, logs: Vec<Log>) -> Self {
        Self {
            returns,
            gas_used,
            logs,
        }
    }

    pub fn take_logs(&mut self) -> Vec<Log> {
        std::mem::take(&mut self.logs)
    }

    pub fn gas_used(&self) -> Gas {
        self.gas_used
    }
}

impl<T: Default> Outcome<T> {
    pub fn take_returns(&mut self) -> T {
        std::mem::take(&mut self.returns)
    }
}

impl<T: Copy> Outcome<T> {
    pub fn returns(&self) -> T {
        self.returns
    }
}
