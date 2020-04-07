use parity_wasm::elements::Instruction;

use crate::block::OpsBlock;
use crate::function::FuncIndex;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Op {
    Plain(Instruction),
    Block(OpsBlock),
    IfBlock(OpsBlock),
    IfElseBlock(OpsBlock, OpsBlock),
    VMCall(FuncIndex),
    FuncCall(FuncIndex),
}
