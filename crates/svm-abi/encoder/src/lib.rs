#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use svm_sdk::{
    types::Primitive,
    value::{Address, Blob1, Blob2, Blob3, PubKey256},
};

pub trait Encoder {
    fn encode(&self, buf: &mut Vec<u8>);
}

impl<'a> Encoder for Address<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(Primitive::Address.into());

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for PubKey256<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(Primitive::PubKey256.into());

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for Blob1<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(Primitive::Blob1.into());

        assert!(buf.len() < std::u8::MAX as usize);
        buf.push(buf.len() as u8);

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for Blob2<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(Primitive::Blob2.into());

        assert!(buf.len() < std::u16::MAX as usize);

        let len_bytes = (buf.len() as u16).to_be_bytes();
        buf.extend_from_slice(&len_bytes);

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for Blob3<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(Primitive::Blob3.into());

        assert!(buf.len() < (1 << 24));

        let len_bytes = (buf.len() as u32).to_be_bytes();
        buf.extend_from_slice(&len_bytes[1..]);

        buf.extend_from_slice(&self.0[..])
    }
}
