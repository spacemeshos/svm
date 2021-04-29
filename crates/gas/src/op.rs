use parity_wasm::elements::Instruction;

use crate::{Block, FuncIndex};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Op {
    Instruction(Instruction),
    Block(Block),
    IfBlock(Block),
    IfElseBlock(Block, Block),
    HostCall(FuncIndex),
    LocalCall(FuncIndex),
}
