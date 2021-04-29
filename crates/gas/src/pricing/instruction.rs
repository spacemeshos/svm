use parity_wasm::elements::Instruction;

use crate::{FuncIndex, Gas, ImportPriceResolver, Imports};

use super::FuncPrice;

pub trait InstructionPriceResolver {
    fn price_for(&self, instruction: &Instruction) -> Gas;
}

pub struct V0PriceResolver {
    func_price: FuncPrice,
}

impl V0PriceResolver {
    pub fn new(func_price: FuncPrice) -> Self {
        Self { func_price }
    }

    pub fn price_for(&self, instruction: &Instruction) -> Gas {
        match instruction {
            Instruction::Block(..) => unreachable!(),
            Instruction::Loop(..) => unreachable!(),
            Instruction::If(..) => unreachable!(),
            Instruction::CallIndirect(..) => unreachable!(),
            Instruction::Call(func) => {
                debug_assert!((*func as usize) < std::u16::MAX as usize);

                let func = FuncIndex(*func as u16);

                self.func_price.get_price(func)

                // let (module, name) = self.imports.get_import(func);

                // self.import_resolver.price_for(module, name)
            }
            _ => Gas::Fixed(1),
        }
    }
}
