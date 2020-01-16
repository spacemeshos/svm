#[derive(Debug, Clone, PartialEq)]
pub struct BufferSlice {
    pub data: Vec<u8>,
}

impl BufferSlice {
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}
