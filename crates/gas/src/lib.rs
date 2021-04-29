#![deny(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate is responsible for doing gas validation & estimation for transactions.

mod block;
pub(crate) use block::{Block, BlockContext, FuncsBlocks};

mod estimate;
mod program_reader;
pub use estimate::estimate_code;

mod call_graph;
pub(crate) use call_graph::CallGraph;

mod program;
pub(crate) use program::Program;

mod validation;
pub use validation::validate_wasm;

mod gas;
pub use gas::Gas;

mod function;
pub use function::FuncIndex;
pub(crate) use function::{FuncBody, FuncGas};

mod error;
pub use error::ProgramError;

/// This is the place for the crate traits
pub mod traits;

mod op;
pub(crate) use op::Op;
