#![no_std]
#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

extern crate alloc;
use crate::alloc::vec::Vec;

use svm_sdk::{
    types::{marker, Composite, Primitive, Type},
    value::{Address, AddressOwned, Blob1, Blob2, Blob3, PubKey256, PubKey256Owned},
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

impl<'a> Encoder for Blob1<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::BLOB_1);

        assert!(buf.len() < core::u8::MAX as usize);

        buf.push(buf.len() as u8);
        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for Blob2<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::BLOB_2);

        assert!(buf.len() < core::u16::MAX as usize);

        let len_bytes = (buf.len() as u16).to_be_bytes();
        buf.extend_from_slice(&len_bytes);

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for Blob3<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::BLOB_3);

        assert!(buf.len() < (1 << 24));

        let len_bytes = (buf.len() as u32).to_be_bytes();
        buf.extend_from_slice(&len_bytes[1..]);

        buf.extend_from_slice(&self.0[..])
    }
}

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
