use thiserror::Error;

use std::fmt;

use svm_program::FuncIndex;

use crate::{GraphCycles, NodeLabel};

/// Represents error that may occur while doing gas estimation
#[derive(Debug, PartialEq, Clone, Error)]
pub enum FixedGasError<T = FuncIndex>
where
    T: NodeLabel,
{
    CallIndirectNotAllowed,
    /// `loop` isn't allowed
    LoopNotAllowed,
    /// Recursive calls aren't allowed
    RecursiveCall {
        /// Function containing the recursive-call
        func: T,
        /// The `call` instruction offset relative to the beginning of the function
        offset: usize,
    },
    /// Calls cycles (e.g `A -> B -> C -> A`) aren't allowed
    CallCycle(GraphCycles<T>),
}

impl<T> fmt::Display for FixedGasError<T>
where
    T: NodeLabel,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
