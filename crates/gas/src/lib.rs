#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate is responsible for doing gas validation & estimation for transactions.

mod block;
pub(crate) use block::{Block, BlockContext};

mod import;
mod read;

pub use import::Imports;

// mod pricing;
// pub use pricing::{price_wasm, ImportPriceResolver};

mod call_graph;
pub(crate) use call_graph::{CallGraph, CallGraphBuilder};

mod program;
pub(crate) use program::Program;

mod validation;
pub use validation::validate_wasm;

mod gas;
pub use gas::Gas;

mod function;
pub(crate) use function::FuncBody;
pub use function::FuncIndex;

mod error;
pub use error::ProgramError;

mod op;
pub(crate) use op::Op;
