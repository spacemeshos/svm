#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

#[cfg(test)]
mod tests {
    use svm_abi_encoder::Encoder;
    use svm_sdk::types::Type;
    use svm_sdk::value::{Address, Blob1, Blob2, Blob3, PubKey256, Slice};

    use svm_abi_decoder::{DecodeError, Decoder};

    #[test]
    fn encode_decode_addr() {
        let bytes: [u8; 20] = [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
            0xF0, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        ];
        let addr = Address(&bytes);

        let mut buf = Vec::new();
        addr.encode(&mut buf);

        let mut decoder = Decoder::new(&buf);
        let value = decoder.decode_value().unwrap();

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

        let mut decoder = Decoder::new(&buf);
        let value = decoder.decode_value().unwrap();

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
        addrs.as_slice().encode(&mut buf);

        let mut decoder = Decoder::new(&buf);
        let value = decoder.decode_value().unwrap();
        dbg!(value);
    }
}
