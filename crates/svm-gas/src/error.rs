use crate::function::FuncIndex;

/// Represents error that may occur while doing gas estimation
#[derive(Debug, PartialEq)]
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

    /// `br` isn't allowed
    BrNotAllowed,

    /// `br_if` isn't allowed
    BrIfNotAllowed,

    /// `br_table` isn't allowed
    BrTableNotAllowed,

    /// Recursive calls aren't allowed
    RecursiveCall(Vec<FuncIndex>),
}
