//! This crate tests the encoding & decoding of a function buffer.
//! using SVM default ABI.

#![deny(missing_docs)]
#![deny(unused)]
#![deny(dead_code)]
#![deny(unreachable_code)]

#[cfg(test)]
mod tests {
    use svm_abi_encoder::Encoder;
    use svm_sdk::value::{Address, AddressOwned, Composite, Primitive, PubKey256, Value};

    use svm_abi_decoder::{Cursor, Decoder};

    #[test]
    fn owned_addr_deref() {
        let bytes: [u8; 20] = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
            0xF0, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        ];

        let owned = AddressOwned(bytes);
        let borrowed = owned.deref();

        assert_eq!(borrowed.0, &bytes);
    }

    #[test]
    fn encode_decode_addr() {
        let bytes: [u8; 20] = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
            0xF0, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        ];
        let addr = Address(&bytes);

        let mut buf = Vec::new();
        addr.encode(&mut buf);

        let mut cursor = Cursor::new(&buf);
        let decoder = Decoder::new();
        let value = decoder.decode_value(&mut cursor).unwrap();

        let addr = value.as_addr().unwrap();
        assert_eq!(addr.as_slice(), &bytes);
    }

    #[test]
    fn encode_decode_pubkey256() {
        let bytes: [u8; 32] = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
            0xF0, 0xAA, 0xAA, 0xF0, 0xE0, 0xD0, 0xC0, 0xB0, 0xA0, 0x90, 0x80, 0x70, 0x60, 0x50,
            0x40, 0x30, 0x20, 0x10,
        ];
        let pkey = PubKey256(&bytes);

        let mut buf = Vec::new();
        pkey.encode(&mut buf);

        let mut cursor = Cursor::new(&buf);
        let decoder = Decoder::new();
        let value = decoder.decode_value(&mut cursor).unwrap();

        let pkey = value.as_pubkey256().unwrap();
        assert_eq!(pkey.as_slice(), &bytes);
    }

    #[test]
    fn encode_decode_addr_array() {
        let addr1 = Address(&[0x10; 20]);
        let addr2 = Address(&[0x20; 20]);
        let addr3 = Address(&[0x30; 20]);

        let addrs = vec![addr1, addr2, addr3];

        let mut buf = Vec::new();
        addrs.encode(&mut buf);

        let mut cursor = Cursor::new(&buf);
        let decoder = Decoder::new();
        let value = decoder.decode_value(&mut cursor).unwrap();

        let vec = vec![
            Value::Primitive(Primitive::Address(Address(&[0x10; 20]))),
            Value::Primitive(Primitive::Address(Address(&[0x20; 20]))),
            Value::Primitive(Primitive::Address(Address(&[0x30; 20]))),
        ];

        assert_eq!(value, Value::Composite(Composite::Array(&vec[..])));
    }

    #[test]
    fn encode_decode_pubkey256_array() {
        let pkey1 = PubKey256(&[0x10; 32]);
        let pkey2 = PubKey256(&[0x20; 32]);
        let pkey3 = PubKey256(&[0x30; 32]);

        let pkeys = vec![pkey1, pkey2, pkey3];

        let mut buf = Vec::new();
        pkeys.encode(&mut buf);

        let mut cursor = Cursor::new(&buf);
        let decoder = Decoder::new();
        let value = decoder.decode_value(&mut cursor).unwrap();

        let vec = vec![
            Value::Primitive(Primitive::PubKey256(PubKey256(&[0x10; 32]))),
            Value::Primitive(Primitive::PubKey256(PubKey256(&[0x20; 32]))),
            Value::Primitive(Primitive::PubKey256(PubKey256(&[0x30; 32]))),
        ];

        assert_eq!(value, Value::Composite(Composite::Array(&vec[..])));
    }

    #[test]
    fn display_addr() {
        let addr = Address(&[
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0x11, 0x22, 0x33, 0x44,
            0x55, 0x66, 0x77, 0x88, 0x99, 0xAA,
        ]);

        let s = format!("{}", addr);
        assert_eq!(s, "102030405060708090a0112233445566778899aa");
    }
}
