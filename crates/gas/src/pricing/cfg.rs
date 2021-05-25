use indexmap::IndexMap;
use parity_wasm::elements::Instruction;

use crate::{
    Block, BlockNum, Edge, FuncIndex, FuncPrice, Graph, GraphBuilder, Imports, NodeLabel,
    NodeWeight, Op, PriceResolver, WeightedGraph, WeightedPath, CFG,
};

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
