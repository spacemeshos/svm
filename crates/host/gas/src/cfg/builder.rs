use std::collections::hash_map::Entry;
use std::collections::HashMap;

use parity_wasm::elements::Instruction;

use crate::cfg::{BlockBuilder, BlockRef, Jump};

use super::{
    Block, BlockNum, Cont, ContKind, Depth, DepthUnresolvedCont, Edge, Op, UnresolvedJump, CFG,
};

/// Used to build `CFG`
///
/// The entry point of the building algorithm in located under function `build_func_cfg` at `mod.rs`
/// This function invokes methods on the `CFGBuilder` implemented in this file.
///
/// Read `mod.rs` for more detailed information about the algorithm (also contains illustrations).
pub struct CFGBuilder<'f> {
    current_depth: Depth,

    block_num: BlockNum,

    blocks: Vec<BlockRef<'f>>,

    unresolved_jumps: HashMap<Depth, Vec<UnresolvedJump>>,

    unresolved_cont: HashMap<Depth, DepthUnresolvedCont>,
}

impl<'f> CFGBuilder<'f> {
    /// Creates a new `CFG` builder
    pub fn new() -> Self {
        let start_block = BlockBuilder::new(BlockNum(0));

        let mut builder = Self {
            block_num: BlockNum(0),
            current_depth: Depth(0),
            blocks: vec![BlockRef::NotReady(start_block)],
            unresolved_jumps: HashMap::new(),
            unresolved_cont: HashMap::new(),
        };

        builder.enter_scope(None);

        debug_assert_eq!(builder.current_depth(), Depth(1));

        let block_num = builder.create_block();
        debug_assert_eq!(block_num, BlockNum(1));

        builder.add_cont(BlockNum(0), BlockNum(1), ContKind::Default);

        builder
    }

    /// Returns the currently built `BlockNum`
    pub fn current_block(&self) -> BlockNum {
        self.block_num
    }

    /// Returns the code's current scope `Depth`
    pub fn current_depth(&self) -> Depth {
        self.current_depth
    }

    /// Returns the code's current scope `Depth` minus one
    pub fn parent_depth(&self) -> Depth {
        let depth = self.current_depth();

        debug_assert!(depth > Depth(0));

        depth - 1u32
    }

    /// Appending op `op` to the currently built `Block`
    pub fn append(&mut self, op: Op<'f>) {
        let block = self.current_block_mut();

        // dbg!("Appending op {:?} (Block #{:?})", &op, block.block_num());

        block.append(op);
    }

    /// Entering a new `Scope`. Incrementing the `current depth`
    pub fn enter_scope(&mut self, op: Option<Op<'f>>) {
        self.current_depth += 1;

        // dbg!("Entering Scope depth {:?}", self.current_depth().0);

        if let Some(op) = op {
            self.append(op);
        }

        if cfg!(debug_assertions) {
            let depth = self.current_depth();
            let jumps = self.unresolved_jumps_entry(depth);

            matches!(jumps, Entry::Vacant(..));
        }
    }

    /// Handling an `IF` instruction
    pub fn enter_if(&mut self, op: Op<'f>) {
        debug_assert!(matches!(op.raw(), Instruction::If(..)));

        let block = self.current_block();
        let depth = self.current_depth();

        // dbg!("Entering `IF`");

        let entry = self.depth_unresolved_entry(depth);

        let mut unresolved = DepthUnresolvedCont::default();

        unresolved.set_after_if(block);
        unresolved.set_on_if_false(block);

        entry.or_insert(unresolved);

        let new_current = self.create_block();

        self.add_cont(block, new_current, ContKind::OnIfTrue);

        self.enter_scope(None);
    }

    /// Handling an `ELSE` instruction
    pub fn enter_else(&mut self, op: Op<'f>) {
        debug_assert!(matches!(op.raw(), Instruction::Else));

        let block = self.current_block();
        let depth = self.current_depth();

        // dbg!("Entering `ELSE` at depth = {}", depth.0);

        debug_assert!(depth > Depth(1));

        if let Some(unresolved) = self.get_depth_unresolved_mut(self.parent_depth()) {
            let on_false = unresolved.take_on_if_false().unwrap();
            let _ = unresolved.take_after_if();

            debug_assert!(unresolved.is_empty());
            unresolved.set_after_then(block);

            let new_current = self.create_block();

            self.add_cont(on_false, new_current, ContKind::OnIfFalse);
        } else {
            unreachable!()
        }
    }

    /// Handing exit of an `ELSE Block` / `IF Block` / `General Block`
    pub fn exit_scope(&mut self, op: Op<'f>) {
        debug_assert!(matches!(op.raw(), Instruction::End));

        debug_assert!(self.current_depth() > Depth(0));

        // dbg!("Starting Exiting Scope depth {:?}", self.current_depth().0);

        if self.is_within_else() {
            self.exit_else();
        } else if self.is_within_then() {
            self.exit_then();
        } else {
            self.do_exit_scope(true);
        }
    }

    /// Creates a new `Block` and returns its `BlockNum`
    pub fn create_block(&mut self) -> BlockNum {
        self.block_num.inc();

        let block_num = self.current_block();

        // dbg!("Creating Block #{:?}", block_num.0);

        let block = BlockBuilder::new(block_num);
        let block_ref = BlockRef::NotReady(block);

        self.blocks.push(block_ref);

        block_num
    }

    /// Adds a new unresolved-jump associated with `target_depth = depth` starting at `origin`
    pub fn add_jump(&mut self, origin: BlockNum, depth: Depth) {
        // dbg!(
        //     "Adding an unresolved jump. origin = {}, target-depth = {}",
        //     origin.0,
        //     depth.0
        // );

        let jump = UnresolvedJump::new(origin);
        let jumps = self
            .unresolved_jumps_entry(depth)
            .or_insert_with(|| Vec::new());

        jumps.push(jump);
    }

    /// Adds a continuation-edge of input `kind` between blocks `origin` and `target`
    pub fn add_cont(&mut self, origin: BlockNum, target: BlockNum, kind: ContKind) {
        // dbg!(
        //     "Adding a continuation edge {:?} -> {:?} (kind = `{:?}`)",
        //     origin.0,
        //     target.0,
        //     kind
        // );

        let cont = Cont {
            origin,
            target,
            kind,
        };
        let edge = Edge::Cont(cont);

        self.add_edge(edge);
    }

    fn resolve_jumps(&mut self, depth: Depth, create_new_block: bool) {
        // dbg!(
        //     "Starting to address unresolved jumps for depth = {:?}",
        //     depth.0
        // );

        let origin = self.current_block();
        let jumps = self.unresolved_jumps.remove(&depth);

        if let Some(mut jumps) = jumps {
            if jumps.is_empty() {
                // dbg!("There are NO unresolved jumps for depth = {:?}", depth.0);
                return;
            }

            if create_new_block {
                let old_current = self.current_block();
                let new_current = self.create_block();

                // Adding continuation edge between the previous `current block` to the new one
                self.add_cont(old_current, new_current, ContKind::Default);
            }

            let target = self.current_block();

            // For each unresolved jump - we add an edge of kind `jump` between the
            // unresolved-jump's previous `current block` to the new one.
            for jump in jumps.drain(..) {
                // dbg!("Resolving jump {:?} -> {:?}", jump.origin().0, target.0);

                let jump = jump.resolve(target);
                let edge = Edge::Jump(jump);

                self.add_edge(edge);
            }
        }
    }

    #[inline]
    fn block_mut(&mut self, block_num: BlockNum) -> &mut BlockRef<'f> {
        &mut self.blocks[block_num.0]
    }

    #[inline]
    fn current_block_mut(&mut self) -> &mut BlockRef<'f> {
        let block_num = self.current_block();

        self.block_mut(block_num)
    }

    /// Finish the building process and output a `CFG`
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

    #[inline]
    fn get_depth_unresolved(&self, depth: Depth) -> Option<&DepthUnresolvedCont> {
        self.unresolved_cont.get(&depth)
    }

    #[inline]
    fn get_depth_unresolved_mut(&mut self, depth: Depth) -> Option<&mut DepthUnresolvedCont> {
        self.unresolved_cont.get_mut(&depth)
    }

    #[inline]
    fn remove_depth_unresolved(&mut self, depth: Depth) {
        self.unresolved_cont.remove(&depth);
    }

    #[inline]
    fn depth_unresolved_entry(&mut self, depth: Depth) -> Entry<Depth, DepthUnresolvedCont> {
        self.unresolved_cont.entry(depth)
    }

    fn add_edge(&mut self, edge: Edge) {
        let origin = self.block_mut(edge.origin());
        origin.add_outgoing_edge(edge.clone());

        let target = self.block_mut(edge.target());
        target.add_incoming_edge(edge);
    }

    fn is_within_else(&self) -> bool {
        let depth = self.parent_depth();
        let unresolved = self.get_depth_unresolved(depth);

        match unresolved {
            Some(unresolved) => unresolved.has_after_then(),
            None => false,
        }
    }

    fn is_within_then(&self) -> bool {
        let depth = self.parent_depth();
        let unresolved = self.get_depth_unresolved(depth);

        match unresolved {
            Some(unresolved) => unresolved.has_after_if(),
            None => false,
        }
    }

    fn exit_else(&mut self) {
        let depth = self.parent_depth();

        // dbg!("Exiting `ELSE` at depth {:?}", self.current_depth());

        let old_current = self.current_block();
        let new_current = self.create_block();

        let unresolved = self.get_depth_unresolved_mut(depth).unwrap();

        let after_then = unresolved.take_after_then().unwrap();

        debug_assert!(unresolved.is_empty());

        self.remove_depth_unresolved(depth);

        self.add_cont(old_current, new_current, ContKind::AfterElse);
        self.add_cont(after_then, new_current, ContKind::AfterThen);

        self.do_exit_scope(false);
    }

    fn exit_then(&mut self) {
        let depth = self.parent_depth();

        // dbg!("Exiting `THEN` at depth {:?}", self.current_depth());

        let old_current = self.current_block();
        let new_current = self.create_block();

        let unresolved = self.get_depth_unresolved_mut(depth).unwrap();

        let after_if = unresolved.take_after_if().unwrap();
        let _ = unresolved.take_on_if_false();

        debug_assert!(unresolved.is_empty());
        self.remove_depth_unresolved(depth);

        self.add_cont(old_current, new_current, ContKind::AfterThen);
        self.add_cont(after_if, new_current, ContKind::OnIfFalse);

        self.do_exit_scope(false);
    }

    fn do_exit_scope(&mut self, create_new_block: bool) {
        self.current_depth -= 1;

        let depth = self.current_depth();

        // dbg!("Back to Scope depth {:?}", depth.0);

        self.resolve_jumps(depth, create_new_block);
    }
}
