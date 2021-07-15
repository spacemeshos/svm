#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

//! This crate is responsible for doing gas validation & estimation for transactions.

mod call_graph;
mod cfg;
mod detect_loops;
mod errors;
mod gas;
mod graph;
mod pricing;
mod validation;

pub use call_graph::{CallGraph, CallGraphBuilder};
pub use cfg::build_func_cfg;
pub use cfg::{
    Block, BlockBuilder, BlockNum, BlockRef, CFGBuilder, Cont, ContKind, Edge, Jump, CFG,
};
pub use errors::FixedGasError;
pub use gas::Gas;
pub use graph::compute_max_weight_path;
pub use graph::{
    Graph, GraphBuilder, GraphCycles, Node, NodeData, NodeLabel, NodeRef, NodeWeight,
    WeightedGraph, WeightedPath,
};
pub use pricing::{build_weighted_graph, resolvers};
pub use pricing::{FuncPrice, PriceResolver, ProgramPricing};
pub use validation::validate_wasm;

/// Transaction gas pricing utilities.
pub mod transaction {
    /// Calculates the cost of deploying `bytes` as a template.
    pub fn deploy_template_price(bytes: &[u8]) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }

    /// Calculates the cost of spawning a new app with `bytes` as its contents.
    pub fn spawn_app_price(bytes: &[u8]) -> u64 {
        // todo!()
        1000 * (bytes.len() as u64)
    }
}
