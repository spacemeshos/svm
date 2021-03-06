#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate is responsible for doing gas validation & estimation for transactions.

mod graph;
pub use graph::compute_max_weight_path;
pub use graph::{
    Graph, GraphBuilder, GraphCycles, Node, NodeData, NodeLabel, NodeRef, NodeWeight,
    WeightedGraph, WeightedPath,
};

mod read;
pub use read::read_program;

mod call_graph;
pub use call_graph::{CallGraph, CallGraphBuilder};

mod program;
pub use program::{Imports, Program, ProgramVisitor};

mod validation;
pub use validation::validate_wasm;

mod gas;
pub use gas::Gas;

mod function;
pub use function::{FuncIndex, FuncIterator, Function, Op};

mod error;
pub use error::ProgramError;

mod cfg;
pub use cfg::build_func_cfg;
pub use cfg::{
    Block, BlockBuilder, BlockNum, BlockRef, CFGBuilder, Cont, ContKind, Edge, Jump, CFG,
};

mod pricing;
pub use pricing::{build_weighted_graph, resolvers};
pub use pricing::{FuncPrice, PriceResolver, ProgramPricing};
