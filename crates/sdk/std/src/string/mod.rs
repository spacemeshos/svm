mod builder;
mod token;
mod traits;

pub use builder::StringBuilder;
pub use token::Token;
pub use traits::{ToString, ToToken};

use crate::Vec;

pub struct String {
    inner: Vec<u8>,
}

impl String {
    pub fn new(s: &'static str) -> Self {
        let ptr = s.as_ptr();
        let len = s.len();

        unsafe {
            let inner = Vec::from_raw_parts(ptr, len);
            String { inner }
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

#[cfg(feature = "debug")]
impl core::fmt::Debug for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        extern crate std;
        use std::string::String as StdString;

        let ptr = self.as_ptr() as *mut u8;
        let len = self.as_bytes().len();

        let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
        let string = StdString::from_utf8_lossy(bytes);
        string.fmt(f)
    }
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
        sb.push_token(Token::One('H'));
        sb.push_token(Token::Two('e', 'l'));
        sb.push_token(Token::Three('l', 'o', '!'));

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
