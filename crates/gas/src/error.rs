use std::fmt;
use std::usize;

use crate::FuncIndex;

/// Represents error that may occur while doing gas estimation
#[derive(Debug, PartialEq, Clone)]
pub enum ProgramError {
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

    /// Wasm has no `import` section
    MissingImportSection,

    /// Wasm has no `code` section
    MissingCodeSection,

    /// Recursive calls aren't allowed
    RecursiveCall {
        /// Function containing the recursive-call
        func: FuncIndex,

        /// The `call` instruction offset relative to the beginning of the function
        offset: usize,
    },

    /// Calls cycles (e.g `A -> B -> C -> A`) aren't allowed
    CallCycle(Vec<FuncIndex>),
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
