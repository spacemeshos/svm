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
        Self::new_short_inner([byte], true)
    }

    pub fn new_short<const N: usize>(data: [u8; N]) -> Self {
        Self::new_short_inner(data, true)
    }

    pub unsafe fn new_unchecked(data: Vec<u8>) -> Self {
        let length = data.len();

        if length <= 8 {
            let data = data.as_slice();
            let mut bytes = [0u8; 8];

            seq_macro::seq!(N in 0..8 {
                if N < length {
                    bytes[N] = data[N];
                }
            });

            Self::new_short_inner(bytes, false)
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

    fn new_short_inner<const N: usize>(data: [u8; N], safe: bool) -> Self {
        let length = data.len();
        debug_assert!(length <= 8);

        let mut bytes = [0u8; 8];

        seq_macro::seq!(N in 0..8 {
            if N < length {
                let byte = data[N];
                if safe {
                    ensure_ascii(byte)
                }
                bytes[N] = byte;
            }
        });

        String::Short { bytes, length }
    }
}

#[cfg(feature = "debug")]
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
        sb.push_str(&String::new("Hello"));

        let actual = sb.build();
        let expected = String::new("Hello");

        assert_eq!(expected, actual);
    }

    #[test]
    fn string_builder_push_token() {
        let mut sb = StringBuilder::with_capacity(6);
        sb.push_token(ShortString::One(b'H'));
        sb.push_token(ShortString::Two(b'e', b'l'));
        sb.push_token(ShortString::Two(b'l', b'o'));
        sb.push_token(ShortString::One(b'!'));

        let actual = sb.build();
        let expected = String::new("Hello!");

        assert_eq!(expected, actual);
    }

    #[test]
    fn string_builder_multiple_strings() {
        let mut sb = StringBuilder::with_capacity(100);

        sb.push_str(&String::new("Hello"));
        sb.push_str(&String::new(" "));
        sb.push_str(&String::new("World!"));

        let actual = sb.build();
        let expected = String::new("Hello World!");

        assert_eq!(expected, actual);
    }
}
