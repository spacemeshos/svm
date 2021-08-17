mod builder;
mod digit;
mod traits;

pub use builder::StringBuilder;
pub use digit::{DecDigit, HexDigit};
pub use traits::ToString;

use crate::Vec;

/// Fixed-Gas replacement for [`std::string::String`].
pub enum String {
    Long(Vec<u8>),
    Short { bytes: [u8; 8], length: usize },
}

impl String {
    pub fn from_byte(byte: u8) -> Self {
        Self::new_short_inner(&[byte], true)
    }

    pub fn new_short<const N: usize>(data: [u8; N]) -> Self {
        Self::new_short_inner(&data, true)
    }

    pub unsafe fn new_unchecked(data: Vec<u8>) -> Self {
        if data.len() <= 8 {
            Self::new_short_inner(data.as_slice(), false)
        } else {
            String::Long(data)
        }
    }

    /// Returns a raw pointer to the underlying [`String`] first byte.
    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }

    /// Returns a slice view to the underlying bytes.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            String::Long(vec) => vec.as_slice(),
            String::Short { bytes, length } => &bytes[0..*length],
        }
    }

    fn new_short_inner(data: &[u8], safe: bool) -> Self {
        let length = data.len();
        debug_assert!(length <= 8);

        if safe {
            seq_macro::seq!(N in 0..8 {
                if N < length {
                    let byte = data[N];
                    ensure_ascii(byte);
                }
            });
        }

        let mut bytes = [0u8; 8];

        seq_macro::seq!(N in 0..8 {
            if N < length {
                bytes[N] = data[N];
            }
        });

        String::Short { bytes, length }
    }
}

#[cfg(any(test, feature = "debug"))]
impl core::fmt::Debug for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        extern crate std;

        let ptr = self.as_ptr() as *mut u8;
        let len = self.as_bytes().len();

        let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
        let string = std::string::String::from_utf8_lossy(bytes);

        string.fmt(f)
    }
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
impl String {
    pub fn to_std_string(&self) -> std::string::String {
        let bytes = self.as_bytes();

        unsafe { std::string::String::from_utf8_unchecked(bytes.to_vec()) }
    }
}

#[inline]
fn ensure_ascii(byte: u8) {
    crate::ensure!(byte & 0b1000_0000 == 0)
}

#[cfg(test)]
mod tests {
    use core::cmp::PartialEq;

    use super::*;

    impl PartialEq for String {
        fn eq(&self, other: &Self) -> bool {
            self.as_bytes().eq(other.as_bytes())
        }
    }

    #[test]
    fn string_builder_one_string() {
        let mut sb = StringBuilder::with_capacity(5);
        sb.push_str(&String::new_short([b'H', b'e', b'l', b'l', b'o']));

        let s = sb.build().to_std_string();
        assert_eq!(s.as_str(), "Hello");
    }

    #[test]
    fn string_builder_multiple_strings() {
        let mut sb = StringBuilder::with_capacity(6);
        sb.push_str(&String::from_byte(b'H'));
        sb.push_str(&String::new_short([b'e', b'l']));
        sb.push_str(&String::new_short([b'l', b'o']));
        sb.push_str(&String::from_byte(b'!'));

        let s = sb.build().to_std_string();
        assert_eq!(s.as_str(), "Hello!");
    }
}
