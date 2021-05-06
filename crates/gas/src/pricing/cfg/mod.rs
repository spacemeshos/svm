use std::fmt::{self, Debug};

use parity_wasm::elements::Instruction;

mod edge;
pub use edge::Edge;

mod builder;
pub use builder::CFGBuilder;

mod cont;
pub use cont::Cont;

mod op;
pub use op::Op;

use super::{Block, BlockNum, Depth, Function, UnresolvedJump, WasmJump};

pub fn build_func_cfg<'f>(func: &'f Function<'f>) -> CFG<'f> {
    println!("Starting to build CFG for function #{:?}", func.index().0);

    let mut offset = 1;
    let mut builder = CFGBuilder::new();

    debug_assert_eq!(builder.current_block(), BlockNum(1));
    debug_assert_eq!(builder.current_depth(), Depth(1));

    for op in func.iter() {
        match op_kind(&op) {
            OpKind::Jump => on_jump(op, &mut builder),
            OpKind::ScopeStart => on_scope_start(op, &mut builder),
            OpKind::ScopeEnd => on_scope_end(op, &mut builder),
            OpKind::Other => on_general_op(op, &mut builder),
        }

        offset += 1;
    }

    builder.build()
}

enum OpKind {
    Jump,
    ScopeStart,
    ScopeEnd,
    Other,
}

/// The `on_branch` is getting called upon hitting a Wasm branch instruction (can be one of: `br / br_if / br_table`).
///
/// There is one exception and is the `return` Wasm instruction (immediately returning from the current function).
/// The `returns` is a kind of branch instruction - we can view it as **jumping out** of the function.
/// We don't care about the function's caller since the gas pricing works on function granularity.
///
/// In order to simplify the design, we count the `scope depth` starting at 1, when the function's itself is considered to have `depth=0`
/// This will play well with unifying the `branch`-ing and `return` Wasm instructions under a single flow.
///
/// This is the flow for handling a branch (or `return` as explained above...):
///
/// 1. Extract the `target depth` out of the Wasm branch.
///    We compute `target_depth = current_depth - wasm-br-label - 1`
///
///    For `br_table` we compute a `target depth` for each `table item` and for the `default` as well.
///    For `return` instruction we will always set `target depth = 0`
///
/// 2. We add an `unresolved jump` with `origin = current_block` and `depth = target_depth` (derived at 2.)
///    Then we associate the `unresolved jump` with the `current depth - 1`
///
///    Once we'll unwind back to the target scope depth, we'll have a chance to resolve the jumps (we'll figure out the `target block`).
///    The reason we can't right away resolve the `jump` (that's why we name it `unresolved jump`) is because we don't have all the necessary information.
///
///    The future block which the target `CFG Block` is yet to be read...
///    We have no clue what will be it's numeric number and its first instruction `offset`.
///
/// 3. Finalize the `current block`.
///    When we hit a branch instruction we always close the current `CFG Block` - that's the definition of a basic-block.
///
/// 4. Create a new block and make it `current block`
///
fn on_jump<'f>(op: Op<'f>, builder: &mut CFGBuilder<'f>) {
    fn target_depth(current: Depth, label: u32) -> Depth {
        debug_assert!(current > Depth(label));

        current - label - 1u32
    }

    let depth = builder.current_depth();
    let origin = builder.current_block();

    let mut conditional = true;

    let branch: WasmJump = op.raw().into();

    match branch {
        WasmJump::Return => {
            conditional = false;

            // For `return` we explicitly set `target_depth = 0` (i.e jumping off the function)
            add_jump(origin, Depth(0), builder);
        }
        WasmJump::Br(label) => {
            conditional = false;

            add_jump(origin, target_depth(depth, label), builder);
        }
        WasmJump::BrIf(label) => {
            add_jump(origin, target_depth(depth, label), builder);
        }
        WasmJump::BrTable(table) => {
            // Adding an unresolved jump for the `default` label
            add_jump(origin, target_depth(depth, table.default()), builder);

            // Adding an unresolved jump for each `label`
            for label in table.iter() {
                add_jump(origin, target_depth(depth, *label), builder);
            }
        }
    }

    let origin_next = builder.create_block();

    if conditional {
        add_continuation(origin, origin_next, builder);
    }
}

#[inline]
fn add_jump<'f>(origin: BlockNum, target_depth: Depth, builder: &mut CFGBuilder<'f>) {
    builder.add_jump(origin, target_depth);
}

#[inline]
fn add_continuation(origin: BlockNum, target: BlockNum, builder: &mut CFGBuilder) {
    builder.add_continuation(origin, target);
}

#[inline]
fn on_scope_start<'f>(op: Op<'f>, builder: &mut CFGBuilder<'f>) {
    builder.enter_scope(Some(op));
}

#[inline]
fn on_scope_end<'f>(op: Op<'f>, builder: &mut CFGBuilder<'f>) {
    builder.exit_scope(op);
}

/// Should be called as a fallback handler when the last read Wasm instruction didn't match on any of the above rules.
/// In other words, when we didn't read any `branch` instruction neither a new-scope/end-scope we should arrive here.
#[inline]
fn on_general_op<'f>(op: Op<'f>, builder: &mut CFGBuilder<'f>) {
    builder.append(op);
}

#[inline]
fn op_kind(op: &Op) -> OpKind {
    let op = op.raw();

    if is_jump(op) {
        OpKind::Jump
    } else if is_scope_start(op) {
        OpKind::ScopeStart
    } else if is_scope_end(op) {
        OpKind::ScopeEnd
    } else {
        OpKind::Other
    }
}

#[inline]
fn is_jump(op: &Instruction) -> bool {
    matches!(
        op,
        Instruction::Br(..)
            | Instruction::BrIf(..)
            | Instruction::BrTable(..)
            | Instruction::Return
    )
}

#[inline]
fn is_scope_start(op: &Instruction) -> bool {
    matches!(op, Instruction::Block(..) | Instruction::If(..))
}

#[inline]
fn is_scope_end(op: &Instruction) -> bool {
    is_end(op) || is_else(op)
}

#[inline]
fn is_end(op: &Instruction) -> bool {
    matches!(op, Instruction::End)
}

#[inline]
fn is_else(op: &Instruction) -> bool {
    matches!(op, Instruction::Else)
}

#[derive(PartialEq)]
pub struct CFG<'f> {
    pub blocks: Vec<Block<'f>>,
}

impl<'f> CFG<'f> {
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    pub fn get_block(&self, block_num: BlockNum) -> &Block {
        let num = block_num.0 as usize;

        &self.blocks[num]
    }
}

impl<'f> Debug for CFG<'f> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in self.blocks.iter() {
            block.fmt(f)?;

            writeln!(f, "")?;
        }

        Ok(())
    }
}
