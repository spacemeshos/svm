#![no_std]
#![feature(lang_items)]
#![cfg_attr(feature = "alloc", feature(alloc))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

pub mod decoder;

use core::panic::PanicInfo;

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[cfg(not(test))]
extern "C" fn rust_eh_personality() {}

#[cfg(test)]
#[macro_use]
extern crate std;

#[no_std]
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
