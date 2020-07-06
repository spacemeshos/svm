#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

use svm_sdk::{
    types::{marker, Composite, Primitive, Type},
    value::{Address, Blob1, Blob2, Blob3, PubKey256},
};

pub trait Encoder {
    fn encode(&self, buf: &mut Vec<u8>);
}

impl<'a> Encoder for Address<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::ADDRESS);

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for PubKey256<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::PUBKEY_256);

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for Blob1<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::BLOB_1);

        assert!(buf.len() < std::u8::MAX as usize);
        buf.push(buf.len() as u8);

        buf.extend_from_slice(&self.0[..])
    }
}

impl<'a> Encoder for Blob2<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::BLOB_2);

        assert!(buf.len() < std::u16::MAX as usize);

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

impl<'a> Encoder for &[Address<'a>] {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(marker::ARRAY_START);

        for addr in self.iter() {
            addr.encode(buf);
        }

        buf.push(marker::ARRAY_END);
    }
}
