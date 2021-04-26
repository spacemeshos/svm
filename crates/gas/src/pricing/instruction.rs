use parity_wasm::elements::Instruction;

use crate::Gas;
pub trait InstructionPriceResolver {
    fn price_for(&self, instruction: &Instruction) -> Gas;
}
