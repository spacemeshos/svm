pub struct Cursor<'a> {
    pub bytes: &'a [u8],

    pub offset: usize,

    pub length: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        // let slice: &[u8] = unsafe { core::slice::from_raw_parts(bytes, length) };

        Self {
            bytes,
            offset: 0,
            length: bytes.len(),
        }
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.offset >= self.length
    }

    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.is_eof() {
            return None;
        }

        let byte = unsafe { *self.offset_ptr() };
        Some(byte)
    }

    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        let byte = self.peek();
        self.offset += 1;

        byte
    }

    pub fn read_bytes(&mut self, nbytes: usize) -> Option<*const u8> {
        let last_byte_off = self.offset + nbytes - 1;

        if (last_byte_off >= self.length) {
            return None;
        }

        let ptr = unsafe { self.offset_ptr() };
        self.offset += nbytes;

        Some(ptr)
    }

    pub unsafe fn offset_ptr(&self) -> *const u8 {
        self.bytes.as_ptr().add(self.offset)
    }
}
