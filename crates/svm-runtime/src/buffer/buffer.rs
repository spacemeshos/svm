use crate::buffer::BufferMut;

pub struct Buffer {
    buffer: BufferMut,
}

impl Buffer {
    pub fn new(buffer: BufferMut) -> Self {
        Self { buffer }
    }

    #[inline]
    pub fn len(&self) -> i32 {
        self.buffer.len()
    }

    #[inline]
    pub fn read(&self, offset: i32, len: i32) -> &[u8] {
        self.buffer.read(offset, len)
    }
}
