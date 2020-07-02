#![no_std]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

#[cfg(not(feature = "std"))]
pub mod decoder;

#[cfg(test)]
mod test {
    use crate::decoder::{DecodeError, Decoder};

    use svm_sdk::value::Value;

    struct DefaultDecoder {}

    impl Decoder for DefaultDecoder {
        fn decode(&mut self) -> Result<Value, DecodeError> {
            todo!()
        }
    }

    #[test]
    pub fn test1() {
        use svm_sdk::value::Address;

        let bytes = [0; 20];
        let addr: Address = Address(&bytes);
    }
}
