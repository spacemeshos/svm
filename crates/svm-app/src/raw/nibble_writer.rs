use super::{concat_nibbles, Nibble};

pub struct NibbleWriter {
    nibbles: Vec<Nibble>,
}

impl NibbleWriter {
    pub fn new() -> Self {
        Self {
            nibbles: Vec::new(),
        }
    }

    pub fn write(&mut self, nibbles: &[Nibble]) {
        for nib in nibbles.iter() {
            self.nibbles.push(*nib);
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        assert!(self.is_byte_aligned());

        let (bytes, rem) = concat_nibbles(&self.nibbles[..]);
        debug_assert!(rem.is_none());

        bytes
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nibbles.len()
    }

    #[inline]
    pub fn is_byte_aligned(&self) -> bool {
        self.len() % 2 == 0
    }
}
