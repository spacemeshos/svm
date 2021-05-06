use parity_wasm::elements::Instruction;

use crate::pricing::Op;

use super::Function;

pub struct FuncIterator<'f> {
    offset: usize,

    length: usize,

    code: &'f [Instruction],
}

impl<'f> FuncIterator<'f> {
    pub fn new(func: &'f Function) -> Self {
        let code = func.code();

        Self {
            offset: 0,
            length: code.len(),
            code,
        }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.offset >= self.length
    }
}

impl<'f> Iterator for FuncIterator<'f> {
    type Item = Op<'f>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }

        let raw = &self.code[self.offset];

        self.offset += 1;

        let op = Op::new(raw, self.offset, self.is_eof());

        Some(op)
    }
}
