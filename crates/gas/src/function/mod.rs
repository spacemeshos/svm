use parity_wasm::elements::Instruction;

mod index;
mod iter;

pub use index::FuncIndex;
pub use iter::FuncIterator;

/// A `Function` wraps its code instructions.
/// In addition each function has a unique identifier, a.k.a the `function index`
#[derive(Debug)]
pub struct Function<'f> {
    index: FuncIndex,

    code: &'f [Instruction],
}

impl<'f> Function<'f> {
    /// Creates a new struct
    pub fn new(index: FuncIndex, body: &'f [Instruction]) -> Self {
        Self { index, code: body }
    }

    /// Returns the function's unique id
    pub fn index(&self) -> FuncIndex {
        self.index
    }

    /// Returns a slice to the function's code
    pub fn code(&self) -> &'f [Instruction] {
        self.code
    }

    /// Returns an iterator over the function's code
    pub fn iter(&self) -> FuncIterator {
        FuncIterator::new(self)
    }
}
