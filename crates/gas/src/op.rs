use parity_wasm::elements::Instruction;

use crate::{FuncIndex, OpsBlock};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Op {
    Plain(Instruction),
    Block(OpsBlock),
    IfBlock(OpsBlock),
    IfElseBlock(OpsBlock, OpsBlock),
    VMCall(FuncIndex),
    FuncCall(FuncIndex),
}
