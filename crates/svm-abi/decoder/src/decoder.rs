use svm_sdk::{types::Type, value::Value};

#[cfg(test)]
extern crate std as realstd;

pub struct TypeError {
    //
}

pub struct ValueError {
    //
}

pub enum DecodeError {
    TypeError,

    ValueError,
}

// pub enum DecodeError<'a> {
//     MissingBytes(&'a str),

//     InvalidType,
// }

pub trait Decoder {
    fn decode(&mut self) -> Result<Value, DecodeError>;
}
