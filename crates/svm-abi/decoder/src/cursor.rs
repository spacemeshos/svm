/// Used for traversal of the encoded function input (a.k.a `calldata`).
/// It will be used by the `Decoder`.
///
/// By having the isolation between the `Decoder` and `Cursor` we are able
/// to execute a `Decoder` method that receives as a parameter `&mut Cursor`  
/// while the `Decoder` is borrowed as `&self`.
///
/// This separation was born out of a need to comply to the safe Rust ownership rules
/// (see the look under the `decode_array` under `Decoder` as an example).
pub struct Cursor<'a> {
    /// The traversed function buffer
    pub bytes: &'a [u8],

    /// The current pointed-by offset
    pub offset: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new `Cursor` for encoded function buffer `bytes`
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    /// Returns whether cursor has finished traversal
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.offset >= self.bytes.len()
    }

    /// Returns the next looked-at byte without incrementing `offset`
    /// If already pointing at `EOF` - returns `None`.
    #[inline]
    pub fn peek(&self) -> Option<u8> {
        if self.is_eof() {
            return None;
        }

        let byte = unsafe { *self.offset_ptr() };
        Some(byte)
    }

    /// Returns the next looked-at byte and increments the `offset`.
    /// If already pointing at `EOF` - returns `None`.
    #[inline]
    pub fn read_byte(&mut self) -> Option<u8> {
        let byte = self.peek();
        self.offset += 1;

        byte
    }

    /// If there are at laest `nbytes` unprocessed-yet bytes,
    /// returns a raw pointer to the current pointed-by address.
    /// And then, it increments the `offset` by `nbytes`.
    ///
    /// In case there are less then `nbytes` left bytes - returns `None`.
    pub fn read_bytes(&mut self, nbytes: usize) -> Option<*const u8> {
        let last_byte_off = self.offset + nbytes - 1;

        if last_byte_off >= self.bytes.len() {
            return None;
        }

        let ptr = unsafe { self.offset_ptr() };
        self.offset += nbytes;

        Some(ptr)
    }

    /// Returns a raw pointer to the current pointed-at address.
    #[inline]
    unsafe fn offset_ptr(&self) -> *const u8 {
        self.bytes.as_ptr().add(self.offset)
    }
}
