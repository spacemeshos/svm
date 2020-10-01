use core::iter::Iterator;

use crate::{Cursor, DecodeError, Decoder};

use svm_sdk::value::Value;

pub struct CallData {
    cursor: Cursor<'static>,

    decoder: Decoder,
}

impl CallData {
    pub fn new(bytes: &'static [u8]) -> Self {
        Self {
            cursor: Cursor::new(bytes),
            decoder: Decoder::new(),
        }
    }
}

impl Iterator for CallData {
    type Item = Value<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.is_eof() {
            return None;
        }

        let value = self.decoder.decode_value(&mut self.cursor);

        match value {
            Err(err) => panic!("Invalid CallData"),
            Ok(value) => Some(value),
        }
    }
}
