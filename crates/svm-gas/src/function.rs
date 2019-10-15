use parity_wasm::elements::{Instruction, Instructions};

use std::iter::{IntoIterator, Iterator};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct FuncIndex(pub u32);

#[derive(Debug)]
pub struct FuncBody(pub Instructions);

impl FuncBody {
    pub fn to_vec(&self) -> Vec<Instruction> {
        self.0.elements().to_vec()
    }
}
