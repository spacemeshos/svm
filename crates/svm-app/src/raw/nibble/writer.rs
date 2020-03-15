use crate::{nib, raw::helpers};

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

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            let lnib = nib!((byte & 0xF0) >> 4);
            let rnib = nib!(byte & 0x0F);

            self.write(&[lnib, rnib]);
        }
    }

    #[must_use]
    #[inline]
    pub fn into_bytes(mut self) -> Vec<u8> {
        // before calling `self.bytes()` we must make sure
        // that its number of nibbles is even. If it's not, we pad it with one extra nibble.

        if self.is_byte_aligned() == false {
            let padding = nib!(0);
            self.write(&[padding]);
        }

        let (bytes, rem) = concat_nibbles(&self.nibbles[..]);
        debug_assert!(rem.is_none());

        bytes
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.nibbles.len()
    }

    #[must_use]
    #[inline]
    pub fn is_byte_aligned(&self) -> bool {
        self.len() % 2 == 0
    }
}
