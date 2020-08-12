use svm_abi_layout::layout;
use svm_nibble::NibbleIter;
use svm_sdk::value::{self, Address, Value};

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Debug)]
pub enum TypeError {
    MissingTypeKind,

    InvalidTypeKind(u8),

    ProhibitedTypeKind(u8),
}

#[derive(Debug)]
pub enum ValueError {
    NotEnoughBytes,
}

/// Denotes a decode error
#[derive(Debug)]
pub enum DecodeError {
    /// Type decode error
    Type(TypeError),

    /// Value decode error
    Value(ValueError),
}

macro_rules! assert_no_eof {
    ($iter:expr) => {{
        let err = DecodeError::Type(TypeError::MissingTypeKind);

        $iter.ensure_eof::<DecodeError>(err)?;
    }};
}

macro_rules! decode_fixed_primitive {
    ($self:expr, $ty:ident, $n:expr, $iter:expr) => {{
        let ptr = $self.read_bytes($iter, $n)?;

        let bytes = unsafe { core::mem::transmute::<*const u8, &[u8; $n]>(ptr) };
        let addr = $ty(bytes);

        let primitive = value::Primitive::$ty(addr);
        let value = Value::Primitive(primitive);

        Ok(value)
    }};
}

/// Decodes an encoded function buffer back into a `sdk_values::Value`
pub struct Decoder;

impl Decoder {
    /// New instance
    pub fn new() -> Self {
        Self {}
    }

    /// Decodes the next `sdk_types::Value` (primitive or composite) and returns it.
    /// Returns `DecodeError` when decode fails.
    pub fn decode_value(&self, iter: &mut NibbleIter) -> Result<Value, DecodeError> {
        assert_no_eof!(iter);

        let marker = self.read_marker(iter)?;

        match marker {
            layout::ADDRESS => self.decode_addr(iter)?,
            _ => todo!(),
        };

        // let value = match byte {
        //     layout::ARRAY_START => self.decode_array(iter)?,
        //     layout::ADDRESS => self.decode_addr(iter)?,
        //     layout::ARRAY_END => {
        //         return Err(DecodeError::Type(TypeError::ProhibitedTypeKind(byte)))
        //     }
        //     _ => return Err(DecodeError::Type(TypeError::InvalidTypeKind(byte))),
        // };

        // Ok(value)

        todo!()
    }

    fn decode_array(&self, iter: &mut NibbleIter) -> Result<Value, DecodeError> {
        assert_no_eof!(iter);

        let mut next_byte = self.peek(iter)?;
        let mut values = Vec::new();

        while next_byte != layout::ARRAY_END {
            let v = self.decode_value(iter)?;
            values.push(v);

            next_byte = self.peek(iter)?;
        }

        let _ = self.read_byte(iter)?;

        let values = Box::leak(Box::new(values));
        let array = Value::Composite(value::Composite::Array(values));

        self.verify_array(&array)?;

        Ok(array)
    }

    fn decode_addr(&self, iter: &mut NibbleIter) -> Result<Value, DecodeError> {
        decode_fixed_primitive!(self, Address, 20, iter)
    }

    fn verify_array(&self, _value: &Value) -> Result<(), DecodeError> {
        // todo!()
        Ok(())
    }

    #[inline]
    fn read_byte(&self, iter: &mut NibbleIter) -> Result<u8, DecodeError> {
        todo!()
        // iter.read_byte()
        //     .ok_or(DecodeError::Value(ValueError::NotEnoughBytes))
    }

    #[inline]
    fn read_bytes<'a>(
        &self,
        iter: &'a mut NibbleIter,
        nbytes: usize,
    ) -> Result<*const u8, DecodeError> {
        todo!()
        // iter.read_bytes(nbytes)
        //     .ok_or(DecodeError::Value(ValueError::NotEnoughBytes))
    }

    #[inline]
    fn peek(&self, iter: &mut NibbleIter) -> Result<u8, DecodeError> {
        todo!()
        // iter.peek()
        //     .ok_or(DecodeError::Value(ValueError::NotEnoughBytes))
    }

    #[inline]
    fn read_marker(&self, iter: &mut NibbleIter) -> Result<u8, DecodeError> {
        let nib = iter.next();

        if let Some(nib) = nib {
            let byte = nib.inner();
            let has_more = byte & 0b_1000_0000 != 0;

            let marker = if has_more {
                let rnib = iter.next();

                if let Some(rnib) = rnib {
                    let rnib = rnib.inner();
                    let lnib = byte << 4;

                    return Ok(lnib | rnib);
                }
            } else {
                return Ok(nib.inner());
            };
        }

        Err(DecodeError::Value(ValueError::NotEnoughBytes))
    }
}
