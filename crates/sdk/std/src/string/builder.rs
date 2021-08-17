use crate::ensure;
use crate::{String, Vec};

/// Builds a [`String`] compliant with the Fixed-Gas rules.
pub struct StringBuilder {
    bytes: Vec<u8>,
}

impl StringBuilder {
    /// New builder, reserves room for `capacity` bytes.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
        }
    }

    /// Appends a [`String`] to the being built [`String`].
    #[inline(never)]
    pub fn push_str(&mut self, s: &String) {
        let bytes = s.as_bytes();
        ensure!(bytes.len() < 256);

        seq_macro::seq!(N in 0..256 {
            if N < bytes.len() {
                let byte = bytes[N];
                super::ensure_ascii(byte);
                self.bytes.push(byte);
            }
            else {
                // halt immediately
                return;
            }
        });
    }

    /// Finishes the building process and returns the built [`String`].
    pub fn build(self) -> String {
        unsafe { String::new_unchecked(self.bytes) }
    }
}
