use thiserror::Error;

use std::fmt;

use crate::{FuncIndex, GraphCycles, NodeLabel};

/// Represents error that may occur while doing gas estimation
#[derive(Debug, PartialEq, Clone, Error)]
pub enum ProgramError<T = FuncIndex>
where
    T: NodeLabel,
{
    /// Invalid wasm
    InvalidWasm,

    /// Floats not allowed
    FloatsNotAllowed,

    /// Too many function imports
    TooManyFunctionImports,

    /// Function index is too large
    FunctionIndexTooLarge,

    /// `call_indirect` isn't allowed
    CallIndirectNotAllowed,

    /// `loop` isn't allowed
    LoopNotAllowed,

    /// Wasm has no `code` section
    MissingCodeSection,

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

impl<T> fmt::Display for ProgramError<T>
where
    T: NodeLabel,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
