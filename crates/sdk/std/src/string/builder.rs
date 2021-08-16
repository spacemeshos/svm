use crate::ensure;
use crate::{String, Token, Vec};

/// Builds a [`String`] compliant with the Fixed-Gas rules.
pub struct StringBuilder {
    inner: Vec<u8>,
}

impl StringBuilder {
    /// New builder, reserves room for `capacity` bytes.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
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
                self.inner.push(byte);
            }
            else {
                // halt immediately
                return;
            }
        });
    }

    /// Appends a [`Token`] to the being built [`String`].
    pub fn push_token(&mut self, token: Token) {
        match token {
            Token::One(a) => self.inner.push(a),
            Token::Two(a, b) => {
                self.inner.push(a);
                self.inner.push(b);
            }
        }
    }

    /// Finishes the building process and returns the built [`String`].
    pub fn build(self) -> String {
        String { inner: self.inner }
    }
}
