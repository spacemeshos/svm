#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

pub enum DecodeError<'a> {
    MissingBytes(&'a str),
    InvalidType,
}

trait Encoder {
    fn encode<W>(&self, buf: W);
}

// trait Decoder {
//     fn peek(&mut self) -> Result<Type, TypeError>;

//     fn decode(&mut self) -> Result<Value, ValueError>;
// }
