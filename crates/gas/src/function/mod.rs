use parity_wasm::elements::Instruction;

mod index;
mod iter;
mod op;

pub use index::FuncIndex;
pub use iter::FuncIterator;
pub use op::Op;

#[derive(Debug)]
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
