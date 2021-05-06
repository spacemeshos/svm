use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::pricing::{BlockBuilder, BlockRef, Jump};

use super::{Block, BlockNum, Cont, Depth, Edge, Op, UnresolvedJump, CFG};

pub struct CFGBuilder<'f> {
    current_depth: Depth,

    block_num: BlockNum,

    blocks: Vec<BlockRef<'f>>,

    unresolved_jumps: HashMap<Depth, Vec<UnresolvedJump>>,
}

impl<'f> CFGBuilder<'f> {
    pub fn new() -> Self {
        let start_block = BlockBuilder::new(BlockNum(0));

        let mut builder = Self {
            block_num: BlockNum(0),
            current_depth: Depth(0),
            blocks: vec![BlockRef::NotReady(start_block)],
            unresolved_jumps: HashMap::new(),
        };

        builder.enter_scope(None);

        debug_assert_eq!(builder.current_depth(), Depth(1));

        let block_num = builder.create_block();
        debug_assert_eq!(block_num, BlockNum(1));

        builder.add_continuation(BlockNum(0), BlockNum(1));

        builder
    }

    pub fn current_block(&self) -> BlockNum {
        self.block_num
    }

    pub fn current_depth(&self) -> Depth {
        self.current_depth
    }

    pub fn append(&mut self, op: Op<'f>) {
        let block = self.get_current_block_mut();

        println!("Appending op {:?} (Block #{:?})", &op, block.block_num());

        block.append(op);
    }

    // Entering a new Scope happens when visiting one of:
    /// * `block` instruction
    /// * `if`    instruction
    //
    /// Note: the `loop` instruction isn't relevant here. (otherwise the validation should have failed)
    //
    // 1. Increment the `current depth`
    // 2. Assert that the number of `unresolved jumps` under the new scope-depth is zero
    //
    pub fn enter_scope(&mut self, op: Option<Op<'f>>) {
        self.current_depth += 1;

        println!("Entering Scope depth {:?}", self.current_depth().0);

        if let Some(op) = op {
            self.append(op);
        }

        let depth = self.current_depth();
        let jumps = self.unresolved_jumps_entry(depth);

        debug_assert!(matches!(jumps, Entry::Vacant(..)));
    }

    // Exiting the current Scope happens when visiting one of:
    // * `end`  instruction
    // * `else` instruction - actually this instruction has two roles.
    ///   It ends the `then` Scope and it marks the immediate beginning of the `else` Scope
    //    (in case there is an `else` block for the `if` statement).
    //
    // 1. Decrement the `current depth`
    // 2. Create a new block and make it the `current block`
    // 3. Resolve each `unresolved jump` associated with the `current depth`
    //    each resolved jump will target the `current block` (see the `resolve_jumps` method)
    pub fn exit_scope(&mut self, op: Op<'f>) {
        debug_assert!(self.current_depth() > Depth(0));

        println!("Exiting Scope depth {:?}", self.current_depth());

        self.append(op);

        self.current_depth -= 1;

        println!("Back to Scope depth {:?}", self.current_depth().0);

        let depth = self.current_depth();

        self.resolve_jumps(depth);
    }

    pub fn create_block(&mut self) -> BlockNum {
        self.block_num.inc();

        let block_num = self.current_block();
        let depth = self.current_depth();

        println!("Creating Block #{:?} (depth = {:?})", block_num.0, depth.0);

        let block = BlockBuilder::new(block_num);
        let block_ref = BlockRef::NotReady(block);

        self.blocks.push(block_ref);

        block_num
    }

    pub fn add_jump(&mut self, origin: BlockNum, depth: Depth) {
        println!(
            "Adding an unresolved jump. origin = {}, target-depth = {}",
            origin.0, depth.0
        );

        let jump = UnresolvedJump::new(origin);
        let jumps = self
            .unresolved_jumps_entry(depth)
            .or_insert_with(|| Vec::new());

        jumps.push(jump);
    }

    pub fn add_continuation(&mut self, origin: BlockNum, target: BlockNum) {
        println!(
            "Adding a continuation edge {:?} -> {:?}",
            origin.0, target.0
        );

        let cont = Cont { origin, target };
        let edge = Edge::Cont(cont);

        self.add_edge(edge);
    }

    fn resolve_jumps(&mut self, depth: Depth) {
        println!(
            "Starting to address unresolved jumps for depth = {:?}",
            depth.0
        );

        let old_current = self.current_block();
        let jumps = self.unresolved_jumps.remove(&depth);

        if let Some(mut jumps) = jumps {
            if jumps.is_empty() {
                println!("There are NO unresolved jumps for depth = {:?}", depth.0);
                return;
            }

            // each Wasm jump ends the current `basic block`.
            // and a new one, named `target` is created.
            let new_current = self.create_block();

            // Adding continuation edge between the previous `current block` to the new one
            self.add_continuation(old_current, new_current);

            // For each unresolved jump - we add an edge of kind `jump` between the
            // unresolved-jump's previous `current block` to the new one.
            for jump in jumps.drain(..) {
                println!(
                    "Resolving jump {:?} -> {:?}",
                    jump.origin().0,
                    new_current.0
                );

                let jump = jump.resolve(new_current);
                let edge = Edge::Jump(jump);

                self.add_edge(edge);
            }
        }
    }

    #[inline]
    pub fn get_block_mut(&mut self, block_num: BlockNum) -> &mut BlockRef<'f> {
        &mut self.blocks[block_num.0]
    }

    #[inline]
    pub fn get_current_block_mut(&mut self) -> &mut BlockRef<'f> {
        let block_num = self.current_block();

        self.get_block_mut(block_num)
    }

    pub fn build(mut self) -> CFG<'f> {
        debug_assert!(self.unresolved_jumps.is_empty());
        debug_assert_eq!(self.current_depth(), Depth(0));

        let mut blocks = Vec::with_capacity(self.blocks.len());

        for mut block_ref in self.blocks.drain(..) {
            let block_ref = block_ref.replace(BlockRef::Empty);

            let block = match block_ref {
                BlockRef::NotReady(builder) => builder.build(),
                BlockRef::Ready(block) => block,
                BlockRef::Empty => unreachable!(),
            };

            blocks.push(block);
        }

        CFG { blocks }
    }

    #[inline]
    fn unresolved_jumps_entry(&mut self, depth: Depth) -> Entry<Depth, Vec<UnresolvedJump>> {
        self.unresolved_jumps.entry(depth)
    }

    // Adds an edge between the CFG Block `origin` to the `target` CFG Block
    // An edge can be represent either a Wasm branch (or `return`) instruction or a continuation between blocks.
    // TODO: explain what's continuation between blocks
    fn add_edge(&mut self, edge: Edge) {
        let origin = self.get_block_mut(edge.origin());
        origin.add_outgoing_edge(edge.clone());

        let target = self.get_block_mut(edge.target());
        target.add_incoming_edge(edge);
    }
}
