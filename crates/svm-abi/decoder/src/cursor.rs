pub struct Cursor {
    bytes: *const u8,

    offset: usize,

    length: usize,
}

impl Cursor {
    pub fn new(bytes: *const u8, length: usize) -> Self {
        let slice: &[u8] = unsafe { core::slice::from_raw_parts(bytes, length) };

        Self {
            bytes,
            offset: 0,
            length,
        }
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.offset >= self.length
    }

    #[inline]
    pub fn peek(&self) -> u8 {
        unsafe { *self.offset_ptr() }
    }

    #[inline]
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.peek();
        self.offset += 1;

        byte
    }

    pub fn read_bytes(&mut self, nbytes: usize) -> Option<*const u8> {
        let last = self.offset + nbytes - 1;

        if (last >= self.length) {
            return None;
        }

        let ptr = self.offset_ptr();
        self.offset += self.length;

        Some(ptr)
    }

    pub fn offset_ptr(&self) -> *const u8 {
        unsafe { self.bytes.add(self.offset) }
    }
}
