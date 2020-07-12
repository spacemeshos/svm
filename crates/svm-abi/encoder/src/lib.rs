#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;
use crate::alloc::vec::Vec;

use svm_sdk::{
    types::{marker, Composite, Primitive, Type},
    value::{Address, AddressOwned, PubKey256, PubKey256Owned},
};

pub trait Encoder {
    fn encode(&self, buf: &mut Vec<u8>);
}

macro_rules! impl_primitive_encoder {
    ($ty:ty, $marker:path) => {
        impl Encoder for $ty {
            fn encode(&self, buf: &mut Vec<u8>) {
                buf.push($marker);

                buf.extend_from_slice(&self.0[..])
            }
        }
    };
}

impl_primitive_encoder!(Address<'_>, marker::ADDRESS);
impl_primitive_encoder!(AddressOwned, marker::ADDRESS);

impl_primitive_encoder!(PubKey256<'_>, marker::PUBKEY_256);
impl_primitive_encoder!(PubKey256Owned, marker::PUBKEY_256);

impl<'a, T> Encoder for &[T]
where
    T: Encoder,
{
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::ARRAY_START);

        for elem in self.iter() {
            elem.encode(buf);
        }

        buf.push(marker::ARRAY_END);
    }
}

impl<'a, T> Encoder for Vec<T>
where
    T: Encoder,
{
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        (&self[..]).encode(buf)
    }
}
