use indexmap::IndexMap;
use parity_wasm::elements::Instruction;

use svm_program::*;

use crate::{
    Block, BlockNum, Edge, FuncPrice, Graph, GraphBuilder, NodeLabel, NodeWeight, PriceResolver,
    WeightedGraph, WeightedPath, CFG,
};

/// This function's job is to take an input `CFG` and translate it into a `WeightedGraph`.
/// A `WeightedGraph` is a `Graph` having `NodeWeight` for its `data` type.
///
/// In `WeightedGraph` each `Node` has an associated `weight`.
/// In addition to `weight`, a `NodeWeight` has `dependant weight` and `dependant label` fields.
///
/// They are not used inside `build_weighted_graph`. Their serve as assisting fields when
/// computing a maximum-path (or similar) as done in `ProgramPricing` (see `mod.rs`) in order to find
/// the maximum price of a given function.
///
/// The `weight` for each `Node` under the built `WeightedGraph` is computed by summing the prices of each `op`
/// under the original `Block` in the `CFG`. So if some `Block`, let's named it `B0` contained 3 `ops` namely `op_1`, `op_2` and `op_3` -
/// the corresponding `Node` under the `WeightedGraph` will have `weight = price(op_1) + price(op_2) + price(op_3)`
///
/// There is no weight associated with the `Edge`(s) of the `WeightedGraph`.
/// This is an implementation detail. We preferred to use a `Graph` which has no weight in its `Edge`(s)
/// and pick a `Data` type for the `Node`(s) that will contain `weight`.
pub fn build_weighted_graph<R>(
    cfg: &CFG,
    resolver: &R,
    imports: &Imports,
    func_price: &FuncPrice,
) -> WeightedGraph<BlockNum>
where
    R: PriceResolver,
{
    let mut builder = GraphBuilder::new();

    for block in cfg.blocks() {
        let node = builder.get_or_create_mut(block.num());
        let mut node_ref = node.as_mut();

        let price = compute_block_price(block, resolver, imports, func_price);

        let mut data: &NodeWeight<BlockNum> = node_ref.data();

        data.set_label(block.num());
        data.set_weight(price);
    }

    for block in cfg.blocks() {
        for edge in block.outgoing_edges() {
            let origin = edge.origin();
            let target = edge.target();

            builder.add_edge(origin, target);
        }
    }

    builder.build()
}

fn compute_block_price<R>(
    block: &Block,
    resolver: &R,
    imports: &Imports,
    func_price: &FuncPrice,
) -> usize
where
    R: PriceResolver,
{
    block.ops().iter().fold(0, |acc, op| {
        acc + resolve_op(op, resolver, imports, func_price)
    })
}

fn resolve_op<R>(op: &Op, resolver: &R, imports: &Imports, func_price: &FuncPrice) -> usize
where
    R: PriceResolver,
{
    if let Instruction::Call(target) = op.raw() {
        resolve_call_price(op, resolver, imports, func_price)
    } else {
        resolver.op_price(op)
    }
}

fn resolve_call_price<R>(op: &Op, resolver: &R, imports: &Imports, func_price: &FuncPrice) -> usize
where
    R: PriceResolver,
{
    if let Instruction::Call(target) = op.raw() {
        let target = FuncIndex(*target);

        let call_price = if let Some(import) = imports.try_resolve(target) {
            resolver.import_price(import)
        } else {
            func_price.get(target)
        };

        call_price
    } else {
        unreachable!()
    }
}

fn _assert_label_ty() {
    fn assert_ty<L: NodeLabel>() {}

    assert_ty::<BlockNum>()
}
