/// # Building a Function's CFG ([`Control-Flow-Graph`])
///
/// ## Motivation
///
/// The motivation is that once we have a CFG of a function we can analyze its execution paths.
/// We want that in order to be able to give Gas pricing for different execution paths.
/// By having that we can pick the most expensive execution path and set it as the function price.  
///
/// The Gas pricing will be possible since building CFG could be called only after the validation of the Wasm program succeeded.
/// (As a remainder - the validation makes sure there are no `loop / call_indirect` neither `recursions / call-cycles` in the code).             
///
/// ## Terminology
///
/// 1.  `CFG` - The term is a short for [`Control-Flow-Graph`]. In our context we refer to building a CFG for a single function.
///     For more info - see the `Motivation` section above.   
///
/// 2.  `Block` - usually it's called `Basic Block` but we'll just use the name `Block` here.
///     A `Block` is essentially a `Node` in the `CFG`, and it has:
///
///     * `id`  - A unique `id` (see `BlockNum` in the code).
///     * `ops` - A list of code instructions. The instructions are guaranteed not to be branching instructions.
///        Branching instructions are the reasons for stopping appending `instructions` to the current `Block`,
///        and creating a new `Blocks`.
///     * `edges` - Connections between the Blocks in the `CFG`. Each Blocks keeps a list of its incoming edges and another for its outgoing one.
///        There are two types of edges: `continuations` and `jumps`. For more info see later `Edge`, `Continuation` and `Jump` (bullets 5, 6 and 7 respectively).           
///
/// 3. `Scope` - Since Wasm has a structured control-flow (there are no `goto`), each instruction lives under its parent Scope.
///    The analyses of each Scope start and end can be obtained by doing a simple static-analysis.
///    Scopes, like in high-level programming languages can be nested. So Scope `A` can have a child Scope `B` that may have other children `Scope`s and so on.
///
/// 4. `Depth` - The nesting-level of a `Scope`.
///    If `Scope` A starts at `depth = D` and at some points starts a child scope `B` - then `B` starts at `depth = D + 1`
///    Scope with `depth=0` is reserved to the function itself. It a design decisions that simplifies implementing the `return` and `unreachable` instructions
///    in the same mechanism as real branching instructions (`br / br_if / br_table`).
///
/// 5. `Edge` - Connections between `Block`. There are two kinds of `Edge`s: `Continuation` and `Jump`.
///
/// 6. `Continuation` - While constructing the CFG, we create new `Block` and connect them to the previously closed ones.
///    A continuation-edge (or `cont` for short) between Blocks `A` and `B` essentially says there exists a valid execution path from `A` to `B`.
///    In that execution path, after we're finished executing the instructions of Block `A` we are allowed to proceed to Block `B`.
///    Each continuation is accompanied with a `kind` - it main purpose is to assist with story-telling the reason for adding the continuation.
///    (When debugging an emitted CFG, it's very handy to have more information).
///
/// 7. `Jump` - In Wasm there are a couple of branching instructions: `br / br_if / br_table`.
///    A branch instruction can result in `JUMP-ing to other locations in the code. Without going here about the nuances of each branching instruction,
///    we want to draw a `Jump`-edges in the CFG between possible jumps. We are able to do that since there is no arbitrary `goto`(s) in Wasm.
///    The control-flow is structured and we can determine the targets of each branch.
///
///    Note: we treat `return` and `unreachable` the same as branch. We look at it as "jumping" out of the function.
///    This plays well with our design, and that's the reason why we reserve `Scope #0` to the function's entry.
///
///
/// ## Algorithm
///
/// 1. Scan the function's instruction one at a time.
///     1.1 Mark `op` as the current scanned instruction.
///     1.2 If `op` isn't `IF / ELSE / END` or a `jump` instruction:
///         1.2.1 Append `op` to the `current block` instructions
///
/// 2. If `op` is a `jump` instruction:
///     2.1 Mark the current block as `old current`
///     2.2 Create a new block and mark it as `new current`
///     2.3 For each possible jump: (`br_table` may contain multiple `labels`)
///         2.3.1 `target_depth` <- jump target `Depth`  
///         2.3.2 Add an unresolved jump with `origin = old current` and associate to `depth = target_depth`
///     2.4 If `op` is a conditional-jump (in other words `br_if`)
///         2.4.1 Add `default` continuation-edge with `origin = old current` to `target = new current`
///
/// 3. If `op` is an `IF` instruction`
///     3.1 Mark the current block as `old current`
///     3.2 Create a new block and mark it as `new current`
///     3.3 Add a `on-if-true` continuation-edge between blocks `old current` to `new current`
///     3.4 Add unresolved-continuations associated with `depth = current_depth`:
///         3.4.1 Add an unresolved continuation for kind `after-if`    with `origin = old current`
///         3.4.2 Add an unresolved continuation for kind `on-if-false` with `origin = old current`
///
///       Note: At this stage we don't know whether the `IF`-block will have an `ELSE` block - so we prepare for any case.
///    3.5 Increment the `current depth`
///
/// 4. If `op` is an `ELSE` instruction:
///     4.1 Mark the current block as `old current`
///     4.2 Create a new block and mark it as `new current`
///     4.3 `parent_depth <- current_depth - 1`
///     4.3 Extract the `origin` of the `on-if-false` unresolved continuation  associated with `depth = parent_depth`
///         Note: such unresolved continuation exists (see `3.4.2` above).
///     4.4 Add unresolved-continuations associated with `depth = current_depth`:
///         4.4.1 Add an unresolved continuation for kind `after-then` with `origin = old current`
///     4.5 Resolve the unresolved-continuation of kind `on-if-false` (created at `3.4.2`)
///         4.5.1 Assign it `target = new current`.
///         4.5.2 Add a jump-edge with the `origin` and `target` of the resolved continuation of 4.5.1
///
/// 5. If `op` is an `END` instruction:
///     5.1 `parent_depth <- current_depth - 1`
///     5.2 If we're ending an existing `ELSE` block:
///         5.2.1 Mark the current block as `old current`
///         5.2.2 Create a new block and mark it as `new current`
///         5.2.3 Extract the `after-then` block under `unresolved-cont[parent_depth]`
///               Note: the `after-then` block has been assigned previously when we visited the
///               `ELSE` instruction associated with the current `END` (see `4.4` above).
///         5.2.4 Add `after-else` continuation-edge between `old current` to `new current`.
///         5.2.5 Add `after-then` continuation-edge between `after-then` block to `new current`.
///         5.2.6 goto 5.5
///     5.3 If we're ending a `THEN` block:
///         5.3.1 Mark the current block as `old current`
///         5.3.2 Create a new block and mark it as `new current`
///         5.3.3 Extract the `after-if` block under `unresolved-cont[parent_depth]`
///         5.3.4 Add `after-else` continuation-edge between `old current` to `new current`.
///         5.3.5 Add `on-if-false` continuation-edge between `after-if` block to `new current`.
///         5.3.6 goto 5.5
///     5.4 Else (i.e `END`ing a general `Block` but not an `ELSE` or an `THEN`)
///         5.4.1 goto 5.5
///     5.5 Decrement the `current depth`
///     5.6 Resolve `unresolved-jumps` associated with the `current depth` (after the decrement):
///         5.6.1 If we landed here after 5.4 (i.e NOT 5.2 or 5.3)
///             * Mark the current block as `old current`
///             * Create a new block and mark it as `new current`
///             * Add `default` continuation between `old current` to `new current`.
///         5.6.2 For each unresolved jump `jump`:
///             * `target <- current block`
///             * Add a `jump` edge between the `jump`'s `origin` to the `target`
///
/// 6. If `op` is a `BLOCK` instruction:
///     6.1 Increment the `current depth`
///
/// [`Control-Flow-Graph`]: https://en.wikipedia.org/wiki/Control-flow_graph
///
///
/// ### Functions CFG Illustrations
///
/// In order to make it easier to grasp the above algorithm - here a couple of illustrations
/// of how different instructions are translated when building the CFG.
///
/// These illustrations are not exhaustive - there are other interesting cases (for example nested `if-then` blocks).
/// But understanding them should give a good background when trying to understand the code of the CFG building.
///
/// Moreover, there are tests under `tests/cfg.rs` that compare against CFGs described under `tests/graphs.
/// Each file under `tests/graphs` is easy to draw as a graph on paper when debugging tests.
///
/// a) Illustration for handling an `if-then` (without `else`):
///
/// +----------------------+
/// |       block          | depth = `d`
/// +----------------------+      
///        ||    ||                *****************
///        ||    ||                * `on-if-false` *
///        ||    ||_______________ *****************_____
///        ||    |_____________________________________ |
///        ||                                          ||
///        || ****************                         ||
///        || * `on-if-true` *                         ||
///        || ****************                         ||
///        ||                                          ||
///        \/                                          ||
///   +---------------------+                          ||
///   | first `then` block  | depth = `d + 1`          ||
///   +---------------------+                          ||
///        ||                                          ||
///        ||                                          ||
///        \/                                          ||
///       .... (more blocks)                           ||
///        ||                                          ||
///        ||                                          ||
///        \/                                          ||
///   +--------------------+                           ||
///   | last `then` block  | depth = `d + 1`           ||
///   +--------------------+                           ||
///        ||                                          ||
///        ||                                          ||
///        || ****************                         ||
///        || * `after-then` *                         ||
///        || ****************                         ||
///        ||   _______________________________________||
///        ||   | ______________________________________|
///        ||   ||
///        ||   ||
///        \/   \/
/// +-------------------------------+
/// |       `after-if` block        | depth = `d`
/// +-------------------------------+
///
///
/// b) Illustration for handling an `if-then-else`:
///
/// +----------------------+
/// |       block          | depth = `d`
/// +----------------------+      
///        ||    ||                 *****************
///        ||    ||                * `on-if-false` *
///        ||    ||_______________ *****************______________
///        ||    |______________________________________________ |
///        ||                                                   ||
///        ||                                                   ||
///        ||                                                   ||
///        ||                                                   ||
///        || ****************                                  ||
///        || * `on-if-true` *                                  ||
///        || ****************                                  ||
///        ||                                                   ||
///        \/                                                   \/
///   +---------------------+                             +---------------------+                                                
///   | first `then` block  | depth = `d + 1`             | first `else` block  | depth = `d + 1`
///   +---------------------+                             +---------------------+
///        ||                                                   ||
///        ||                                                   ||
///        \/                                                   \/
///       .... (more blocks)                                   .... (more blocks)    
///        ||                                                   ||
///        ||                                                   ||
///        \/                                                   \/
///   +--------------------+                            +---------------------+
///   | last `then` block  | depth = `d + 1`            |  last `else` block  | depth = `d + 1`
///   +--------------------+                            +---------------------+
///        ||                                                   ||
///        ||                                                   ||
///        ||                                                   ||
///        || ****************                                  || ****************
///        || * `after-then` *                                  || * `after-else` *
///        || ****************                                  || ****************
///        ||                                                   ||
///        ||   ________________________________________________||
///        ||   | _______________________________________________|
///        ||   ||
///        ||   ||
///        ||   ||
///        ||   ||
///        ||   ||
///        ||   ||
///        \/   \/
/// +---------------------------+
/// |     `after-if` block      | depth = `d`
/// +---------------------------+
///
///
///
/// c) Illustration for handling `br_if 0`:
///
/// +----------------------+
/// |       block          | depth = `d`
/// +----------------------+      
///        ||    ||                
///        ||    ||           *******************               
///        ||    ||           * `default` cont. *    
///        ||    ||___________*******************________________
///        ||    |______________________________________________ |
///        ||                                                   ||
///        ||                                                   ||
///        || **********                                        ||                   
///        || * `jump` *                                    +------------+
///        || **********                                    |    block   | depth = `d`            
///        ||                                               +------------+
///        ||   ________________________________________________||
///        ||   | _______________________________________________|
///        ||   ||
///        ||   ||
///        ||   ||
///        \/   \/
/// +-----------------------------------+
/// |       `branch target` block       | depth = `d - 1`
/// +-----------------------------------+
///
///
/// d) illustration for handling `br 0`:
///
/// +----------------------+
/// |       block          | depth = `d`
/// +----------------------+      
///        ||   
///        ||
///        ||                                                   
///        || **********                                   
///        || * `jump` *                                  +------------+
///        || **********                                  |    block   | depth = `d`            
///        ||                                             +------------+ (unreachable)
///        ||                                                   ||  
///        ||                                                   ||
///        ||                                                   ||
///        ||_____________________          ____________________||
///        |____________________ |          | ___________________|
///                             ||          ||    *******************
///                             ||          ||    * `default` cont. *
///                             ||          ||    *******************
///                             ||          ||
///                             ||          ||
///                             ||          ||
///                             \/          \/
///                     +-----------------------------------+
///                     |       `branch target` block       | depth = `d - 1`
///                     +-----------------------------------+
///
///
///
use parity_wasm::elements::Instruction;

use svm_program::*;

use std::fmt::{self, Debug};

use crate::{CallGraph, Gas};

mod block;
mod builder;
mod cont;
mod depth;
mod edge;
mod jump;

pub use block::{Block, BlockBuilder, BlockNum, BlockRef};
pub use builder::CFGBuilder;
pub use cont::{Cont, ContKind, DepthUnresolvedCont};
pub use depth::Depth;
pub use edge::Edge;
pub use jump::{Jump, UnresolvedJump, WasmJump};

/// This is the API that should be used externally when we want to feed with a  `Function` and get back its `CFG`
pub fn build_func_cfg<'f>(func: &'f Function<'f>) -> CFG<'f> {
    // println!("Starting to build CFG for function #{:?}", func.index().0);

    let mut builder = CFGBuilder::new();

    debug_assert_eq!(builder.current_block(), BlockNum(1));
    debug_assert_eq!(builder.current_depth(), Depth(1));

    for op in func.iter() {
        match op_kind(&op) {
            OpKind::Jump => on_jump(op, &mut builder),
            OpKind::If => on_if(op, &mut builder),
            OpKind::Else => on_else(op, &mut builder),
            OpKind::ScopeStart => on_scope_start(op, &mut builder),
            OpKind::ScopeEnd => on_scope_end(op, &mut builder),
            OpKind::Other => on_general_op(op, &mut builder),
        }
    }

    // println!(
    //     "Finalizing building the CFG for function #{:?}",
    //     func.index().0
    // );

    builder.build()
}

enum OpKind {
    Jump,
    ScopeStart,
    ScopeEnd,
    If,
    Else,
    Other,
}

/// The `on_branch` is getting called upon hitting a Wasm branch instruction (can be one of: `br / br_if / br_table`).
///
/// There are two exceptions - the `return` and `unreachable` Wasm instruction (immediately returning / halting the current function).
/// The `return / unreachable` are kind of branching instructions - we can view them as **jumping out** of the function.
///
/// In order to simplify the design, we count the `scope depth` starting at 1, when the function's itself is considered to have `depth = 0`
/// This will play well with unifying the `branch`-ing and `return` Wasm instructions under a single flow.
///
/// This is the flow for handling a branch (or `return / unreachable` as explained above.):
///
/// 1. Extract the `target depth` out of the Wasm branch.
///    We compute `target_depth = current_depth - branch-label - 1`
///
///    For `br_table` we compute a `target depth` for each `table item` and for the `default` as well.
///    For `return / unreachable` instructions we will always set `target depth = 0`
///
/// 2. We add an `unresolved jump` with `origin = current_block` and `depth = target_depth` (derived at 2.)
///    Then we associate the `unresolved jump` with the `current depth - 1`
///
///    Once we'll unwind back to the target scope depth, we'll have a chance to resolve the jumps.
///    The reason we can't right away resolve a `jump` (and that's why we name it `unresolved jump`)
///    is because we don't know the target `block` at that point.
///
/// 3. Create a new block and make it the new `current block`
///
fn on_jump<'f>(op: Op<'f>, builder: &mut CFGBuilder<'f>) {
    fn target_depth(current: Depth, label: u32) -> Depth {
        debug_assert!(current > Depth(label));

        current - label - 1u32
    }

    let depth = builder.current_depth();
    let origin = builder.current_block();
    let origin_next = builder.create_block();

    let mut conditional = true;

    let branch: WasmJump = op.raw().into();

    match branch {
        WasmJump::Return | WasmJump::Unreachable => {
            // For `return` and `unreachable` we explicitly set `target_depth = 0` (i.e jumping off the function)
            add_jump(origin, Depth(0), builder);
        }
        WasmJump::Br(label) => {
            add_jump(origin, target_depth(depth, label), builder);
        }
        WasmJump::BrIf(label) => {
            add_jump(origin, target_depth(depth, label), builder);

            add_continuation(origin, origin_next, builder);
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
}

#[inline]
fn add_jump<'f>(origin: BlockNum, target_depth: Depth, builder: &mut CFGBuilder<'f>) {
    builder.add_jump(origin, target_depth);
}

#[inline]
fn add_continuation(origin: BlockNum, target: BlockNum, builder: &mut CFGBuilder) {
    builder.add_cont(origin, target, ContKind::Default);
}

#[inline]
fn on_if<'f>(op: Op<'f>, builder: &mut CFGBuilder<'f>) {
    builder.enter_if(op);
}

#[inline]
fn on_else<'f>(op: Op<'f>, builder: &mut CFGBuilder<'f>) {
    builder.enter_else(op);
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
    } else if is_if(op) {
        OpKind::If
    } else if is_else(op) {
        OpKind::Else
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
            | Instruction::Unreachable
    )
}

#[inline]
fn is_scope_start(op: &Instruction) -> bool {
    matches!(op, Instruction::Block(..))
}

#[inline]
fn is_if(op: &Instruction) -> bool {
    matches!(op, Instruction::If(..))
}

#[inline]
fn is_scope_end(op: &Instruction) -> bool {
    matches!(op, Instruction::End)
}

#[inline]
fn is_else(op: &Instruction) -> bool {
    matches!(op, Instruction::Else)
}

/// Since an `Block` is self-contained in the sense it has all its relevant data
/// The `CFG` is merely a container of its `Block`s.
#[derive(PartialEq)]
pub struct CFG<'f> {
    /// The `Block`s of the `CFG`
    pub blocks: Vec<Block<'f>>,
}

impl<'f> CFG<'f> {
    /// Returns a slice to the `CFG` blocks
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Borrows a `Block` with the specified `block_num` parameter
    pub fn get_block(&self, block_num: BlockNum) -> &Block {
        let num = block_num.0 as usize;

        &self.blocks[num]
    }

    /// The `BlockNum` of the entry `Node` starting each flow
    pub fn start(&self) -> BlockNum {
        BlockNum(0)
    }

    /// The `BlockNum` of the last created `Node` ending each flow
    pub fn end(&self) -> BlockNum {
        let len = self.blocks().len();

        debug_assert!(len > 0);

        let end = len - 1;

        BlockNum(end)
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
