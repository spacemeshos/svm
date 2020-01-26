use crate::buffer::BufferMut;

pub struct Buffer {
    buffer: BufferMut,
}

impl Buffer {
    pub fn new(buffer: BufferMut) -> Self {
        Self { buffer }
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.buffer.len()
    }

    #[inline]
    pub fn read(&self, offset: u32, len: u32) -> &[u8] {
        self.buffer.read(offset, len)
    }
}
