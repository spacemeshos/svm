use parity_wasm::elements::Instruction;

use crate::FuncIndex;

mod iter;

pub use iter::FuncIterator;

pub struct Function<'f> {
    index: FuncIndex,

    code: &'f [Instruction],
}

impl<'f> Function<'f> {
    pub fn new(index: FuncIndex, body: &'f [Instruction]) -> Self {
        Self { index, code: body }
    }

    pub fn index(&self) -> FuncIndex {
        self.index
    }

    pub fn code(&self) -> &'f [Instruction] {
        self.code
    }

    pub fn iter(&self) -> FuncIterator {
        FuncIterator::new(self)
    }
}
