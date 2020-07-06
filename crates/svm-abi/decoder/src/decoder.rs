#![feature(alloc)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use svm_sdk::{
    types::{marker, Type},
    value::{self, Address, Blob1, Blob2, Blob3, PubKey256, Value},
};

use crate::cursor::Cursor;

#[derive(Debug)]
pub enum TypeError {
    MissingTypeKind,

    InvalidTypeKind,
}

#[derive(Debug)]
pub enum ValueError {
    NotEnoughBytes,
}

#[derive(Debug)]
pub enum DecodeError {
    Type(TypeError),

    Value(ValueError),
}

macro_rules! assert_no_eof {
    ($cursor:expr) => {{
        if $cursor.is_eof() {
            return Err(DecodeError::Type(TypeError::MissingTypeKind));
        }
    }};
}

pub struct Decoder {}

impl Decoder {
    pub fn decode_value(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        assert_no_eof!(cursor);

        let byte = cursor.read_byte();

        let value = match byte {
            marker::ARRAY_START => self.decode_array(cursor)?,
            marker::TUPLE_START => self.decode_tuple(cursor)?,
            marker::ADDRESS => self.decode_addr(cursor)?,
            marker::PUBKEY_256 => self.decode_pubkey256(cursor)?,
            marker::BLOB_1 => todo!("Blob1"),
            marker::BLOB_2 => todo!("Blob2"),
            marker::BLOB_3 => todo!("Blob3"),
            marker::ARRAY_END => panic!("invalid encoding"),
            marker::TUPLE_END => panic!("invalid encoding"),
            _ => unreachable!(),
        };

        Ok(value)
    }

    fn decode_array(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        assert_no_eof!(cursor);

        let mut next_byte = cursor.peek();
        let mut vec: Vec<Value> = Vec::new();

        while next_byte != marker::ARRAY_END {
            let value = self.decode_value(cursor)?;
            vec.push(value);

            next_byte = cursor.peek();
        }

        let _ = cursor.read_byte();

        let vec = Box::leak(Box::new(vec));
        let array = Value::Composite(value::Composite::Array(vec));

        self.verify_array(&array);

        Ok(array)
    }

    fn decode_tuple(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        todo!()
    }

    fn decode_addr(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        // let ptr = self.read_bytes(20)?;
        // let bytes = unsafe { core::mem::transmute::<*const u8, &[u8; 20]>(ptr) };
        // let addr = Address(bytes);

        // let primitive = value::Primitive::Address(addr);
        // let value = Value::Primitive(primitive);

        // Ok(value)
        todo!()
    }

    fn decode_pubkey256(&self, cursor: &mut Cursor) -> Result<Value, DecodeError> {
        // let ptr = self.read_bytes(32)?;
        // let bytes = unsafe { core::mem::transmute::<*const u8, &[u8; 32]>(ptr) };
        // let pubkey = PubKey256(bytes);

        // let primitive = value::Primitive::PubKey256(pubkey);
        // let value = Value::Primitive(primitive);

        // Ok(value)
        todo!()
    }

    fn verify_array(&self, value: &Value) -> Result<(), DecodeError> {
        todo!()
    }
}
