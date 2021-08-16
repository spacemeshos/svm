use crate::ensure;
use crate::{String, Token, Vec};

pub struct StringBuilder {
    inner: Vec<u8>,
}

impl StringBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
        }
    }

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

    pub fn push_token(&mut self, token: Token) {
        match token {
            Token::One(a) => self.inner.push(a),
            Token::Two(a, b) => {
                self.inner.push(a);
                self.inner.push(b);
            }
            Token::Three(a, b, c) => {
                self.inner.push(a);
                self.inner.push(b);
                self.inner.push(c);
            }
        }
    }

    pub fn build(self) -> String {
        String { inner: self.inner }
    }
}
