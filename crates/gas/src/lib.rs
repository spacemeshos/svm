#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate is responsible for doing gas validation & estimation for transactions.

mod import;
pub use import::Imports;

mod read;
pub use read::read_program;

mod call_graph;
pub use call_graph::{CallGraph, CallGraphBuilder};

mod program;
pub use program::Program;

mod validation;
pub use validation::validate_wasm;

mod gas;
pub use gas::Gas;

mod function;
pub use function::{FuncIndex, FuncIterator, Function};

mod error;
pub use error::ProgramError;

mod cfg;
pub use cfg::{
    build_func_cfg, Block, BlockBuilder, BlockNum, BlockRef, CFGBuilder, Cont, ContKind, Edge,
    Jump, Op, CFG,
};
