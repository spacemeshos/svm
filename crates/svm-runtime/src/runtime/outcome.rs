use core::panic;
use std::{hint::unreachable_unchecked, iter::Successors, unreachable};

use svm_types::gas::MaybeGas;
use svm_types::receipt::Log;
use svm_types::{RuntimeError, State};

use crate::Runtime;

pub enum Outcome<T = Box<[wasmer::Val]>> {
    Success {
        returns: T,

        gas_used: MaybeGas,

        logs: Vec<Log>,
    },
    Failure {
        err: RuntimeError,

        logs: Vec<Log>,
    },
}

impl<T> Outcome<T> {
    pub fn map<S, F>(self, f: F) -> Outcome<S>
    where
        F: Fn(T) -> S,
    {
        match self {
            Outcome::Failure { err, logs } => Outcome::Failure { err, logs },
            Outcome::Success {
                logs,
                gas_used,
                returns,
            } => Outcome::Success {
                logs,
                gas_used,
                returns: f(returns),
            },
        }
    }
}

impl<T> Outcome<T> {
    pub fn take_logs(&mut self) -> Vec<Log> {
        match self {
            Self::Success { ref mut logs, .. } | Self::Failure { ref mut logs, .. } => {
                std::mem::take(logs)
            }
        }
    }

    pub fn returns(&self) -> &T {
        match *self {
            Outcome::Success { ref returns, .. } => returns,
            Outcome::Failure { .. } => unreachable!(),
        }
    }

    pub fn gas_used(&self) -> MaybeGas {
        match *self {
            Outcome::Success { gas_used, .. } => gas_used,
            Outcome::Failure { .. } => unreachable!(),
        }
    }
}

impl<T> From<RuntimeError> for Outcome<T> {
    fn from(err: RuntimeError) -> Self {
        Self::Failure {
            err,
            logs: Vec::new(),
        }
    }
}
