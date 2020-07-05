use svm_sdk::{
    types::Type,
    value::{self, Address, Blob1, Blob2, Blob3, PubKey256, Value},
};

#[derive(Debug)]
pub enum TypeError {
    MissingTypeKind,
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

pub struct Decoder<'a> {
    bytes: &'a [u8],

    cursor: usize,

    length: usize,
}

macro_rules! assert_no_eof {
    ($self:expr) => {{
        if $self.is_eof() {
            return Err(DecodeError::Type(TypeError::MissingTypeKind));
        }
    }};
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            cursor: 0,
            length: bytes.len(),
        }
    }

    pub fn decode(&mut self) -> Result<Value, DecodeError> {
        assert_no_eof!(self);

        let byte = self.read_byte();

        let value = match byte {
            0 => todo!(),
            1 => todo!(),
            2 => todo!(),
            3 => todo!(),
            4 => todo!(),
            5 => self.decode_addr()?,
            6 => todo!(),
            _ => todo!(),
        };

        Ok(value)
    }

    fn decode_composite(&mut self) -> Result<Type, DecodeError> {
        todo!()
    }

    fn decode_value(&mut self) -> Result<Value, DecodeError> {
        todo!()
    }

    fn decode_addr(&mut self) -> Result<Value, DecodeError> {
        let ptr = self.read_bytes(20)?;
        let bytes = unsafe { core::mem::transmute::<*const u8, &[u8; 20]>(ptr) };
        let addr = Address(bytes);

        let primitive = value::Primitive::Address(addr);
        let value = Value::Primitive(primitive);

        Ok(value)
    }

    fn decode_pubkey256(&mut self) -> Result<Value, DecodeError> {
        let ptr = self.read_bytes(32)?;
        let bytes = unsafe { core::mem::transmute::<*const u8, &[u8; 32]>(ptr) };
        let pubkey = PubKey256(bytes);

        let primitive = value::Primitive::PubKey256(pubkey);
        let value = Value::Primitive(primitive);

        Ok(value)
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.cursor >= self.length
    }

    #[inline]
    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.cursor];
        self.cursor += 1;

        byte
    }

    fn read_bytes(&mut self, nbytes: usize) -> Result<*const u8, DecodeError> {
        let last = self.cursor + nbytes - 1;

        if (last >= self.length) {
            return Err(DecodeError::Value(ValueError::NotEnoughBytes));
        }

        let ptr = self.cursor_ptr();
        self.cursor += self.length;

        Ok(ptr)
    }

    fn cursor_ptr(&self) -> *const u8 {
        unsafe { self.bytes.as_ptr().add(self.cursor) }
    }
}
