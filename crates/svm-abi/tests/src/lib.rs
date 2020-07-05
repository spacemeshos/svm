#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]

#[cfg(test)]
mod tests {
    use svm_abi_encoder::Encoder;
    use svm_sdk::types::Type;
    use svm_sdk::value::{Address, Blob1, Blob2, Blob3, PubKey256};

    use svm_abi_decoder::{DecodeError, Decoder};

    #[test]
    fn encode_decode_addr() {
        let bytes: [u8; 20] = [0; 20];
        let addr = Address(&bytes);

        let mut buf = Vec::new();
        addr.encode(&mut buf);

        let mut decoder = Decoder::new(&buf);
        let value = decoder.decode().unwrap();

        dbg!(buf);
    }

    #[ignore]
    #[test]
    fn encode_decode_pubkey256() {
        //
    }

    #[ignore]
    #[test]
    fn encode_decode_blob1() {
        //
    }

    #[ignore]
    #[test]
    fn encode_decode_pubkey256_array() {
        //
    }
}
