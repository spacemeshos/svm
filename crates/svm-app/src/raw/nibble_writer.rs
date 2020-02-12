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

    pub fn as_bytes(&self) -> Vec<u8> {
        let (bytes, rem) = concat_nibbles(&self.nibbles[..]);
        assert!(rem.is_none());

        bytes
    }
}
