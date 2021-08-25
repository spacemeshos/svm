use std::convert::TryInto;

use svm_types::{Account, Context, Envelope, Gas, Layer, SpawnAccount, Transaction};

use crate::version;
use crate::{Codec, Field, ParseError, ReadExt, WriteExt};

#[derive(Debug)]
pub struct InputData(pub Vec<u8>);

/// Encoding of a binary [`Envelope`].
///
/// ```text
///
///  +-------------+--------------+----------------+----------------+
///  |             |              |                |                |
///  |  Principal  |    Amount    |   Gas Limit    |    Gas Fee     |
///  |  (Address)  |    (u64)     |     (u64)      |     (u64)      |
///  |             |              |                |                |
///  |  20 bytes   |   8 bytes    |    8 bytes     |    8 bytes     |
///  |             | (Big-Endian) |  (Big-Endian)  |  (Big-Endian)  |
///  |             |              |                |                |
///  +-------------+--------------+----------------+----------------+
///
/// ```
impl Codec for Envelope {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.principal().0.encode(w);
        self.amount().encode(w);
        self.gas_limit().encode(w);
        self.gas_fee().encode(w);
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let principal = cursor.read_bytes_prim()?;
        let amount = u64::decode(cursor)?;
        let gas_limit = Gas::decode(cursor)?;
        let gas_fee = u64::decode(cursor)?;

        let envelope = Envelope::new(principal, amount, gas_limit, gas_fee);
        Ok(envelope)
    }

    fn fixed_size() -> Option<usize> {
        Some(20 + 8 + 8 + 8)
    }
}

impl<const N: usize> Codec for [u8; N] {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_bytes(&self[..])
    }

    fn decode(cursor: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        Ok((&cursor.read_bytes(N)?[..]).try_into().unwrap())
    }

    fn fixed_size() -> Option<usize> {
        Some(N)
    }
}

/// Encoding of binary [`Transaction`].
///
/// ```text
///
///  +-----------+-------------+----------------+
///  |           |             |                |
///  |  Version  |  Template   |      Name      |
///  |   (u16)   |  (Address)  |    (String)    |
///  |           |             |                |
///  +-----------+-------------+----------------+
///  |           |             |                |
///  | Function  | VerifyData  |    CallData    |
///  | (String)  |   (Blob)    |     (Blob)     |
///  |           |             |                |
///  +-----------+-------------+----------------+
///
/// ```
impl Codec for Transaction {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.version.encode(w);
        self.target().0.encode(w);
        self.func_name().to_string().encode(w);
        InputData(self.verifydata.clone()).encode(w);
        InputData(self.calldata.clone()).encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let version = u16::decode(reader)?;
        let target = <[u8; 20]>::decode(reader)?.into();
        let func_name = String::decode(reader)?;
        let verifydata = InputData::decode(reader)?.0.to_vec();
        let calldata = InputData::decode(reader)?.0.to_vec();

        Ok(Transaction {
            version,
            target,
            func_name,
            verifydata,
            calldata,
        })
    }
}

/// Encoding of a binary [`Context`].
///
/// ```text
///
///  +------------------+-----------------+-----------------+
///  |                  |                 |                 |
///  |  Transaction Id  |  Current Layer  |  Current State  |
///  |     (Hash)       |     (u64)       |     (State)     |
///  |                  |                 |                 |
///  |    32 bytes      |    8 bytes      |    32 bytes     |
///  |                  |   (Big-Endian)  |                 |
///  |                  |                 |                 |
///  +------------------+-----------------+-----------------+
///
/// ```
impl Codec for Context {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.tx_id().0.encode(w);
        self.layer().0.encode(w);
        self.state().0.encode(w);
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let tx_id = <[u8; 32]>::decode(cursor)?.into();
        let layer = u64::decode(cursor)?;
        let state = <[u8; 32]>::decode(cursor)?.into();

        let context = Context::new(tx_id, Layer(layer), state);
        Ok(context)
    }

    fn fixed_size() -> Option<usize> {
        Some(32 + 8 + 32)
    }
}

/// Encoding of a binary [`SpawnAccount`].
///
/// ```text
///
///  +-----------+-------------+----------------+
///  |           |             |                |
///  |  Version  |  Template   |      Name      |
///  |   (u16)   |  (Address)  |    (String)    |
///  |           |             |                |
///  +-----------+-------------+----------------+
///  |           |                              |
///  |   Ctor    |          CallData            |
///  |  (String) |           (Blob)             |
///  |           |                              |
///  +-----------+------------------------------+
///
/// ```
impl Codec for SpawnAccount {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.version.encode(w);
        self.template_addr().0.encode(w);
        self.account_name().to_string().encode(w);
        self.ctor_name().to_string().encode(w);
        InputData(self.calldata.clone()).encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let version = version::decode_version(reader)?;
        let template_addr = reader.read_bytes_prim()?;
        let name = String::decode(reader)?;
        let ctor_name = String::decode(reader)?;
        let calldata = decode_ctor_calldata(reader)?;

        Ok(SpawnAccount {
            version,
            account: Account {
                name,
                template_addr,
            },
            ctor_name,
            calldata,
        })
    }
}

impl Codec for InputData {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        let length = self.0.len();

        assert!(length <= std::u8::MAX as usize);

        w.write_byte(length as u8);
        w.write_bytes(&self.0[..]);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, ParseError> {
        match reader.read_byte() {
            Err(..) => Err(ParseError::NotEnoughBytes(Field::InputDataLength)),
            Ok(byte) => {
                let length = byte as usize;

                reader
                    .read_bytes(length)
                    .map(|x| Self(x))
                    .map_err(|_| ParseError::NotEnoughBytes(Field::InputData))
            }
        }
    }
}

impl Codec for String {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(self.as_bytes().len() as u8);
        w.write_bytes(self.as_bytes());
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let len = reader.read_byte()? as usize;
        let bytes = reader.read_bytes(len)?;

        Ok(String::from_utf8(bytes.to_vec()).map_err(|_| ParseError::Other)?)
    }
}

impl Codec for u16 {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_u16_be(*self);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        reader.read_u16_be()
    }
}

impl Codec for u32 {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_u32_be(*self);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        reader.read_u32_be()
    }
}

impl Codec for u64 {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_u64_be(*self);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        reader.read_u64_be()
    }
}

impl Codec for bool {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_bool(*self);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        reader.read_bool()
    }
}

impl<T> Codec for Vec<T>
where
    T: Codec<Error = ParseError>,
{
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        assert!(self.len() <= u16::MAX as usize);
        w.write_u16_be(self.len() as u16);

        for elem in self {
            elem.encode(w);
        }
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let len = u16::decode(reader)?;
        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            vec.push(T::decode(reader)?);
        }

        Ok(vec)
    }
}

/// Decoders

fn decode_ctor_calldata(cursor: &mut impl ReadExt) -> Result<Vec<u8>, ParseError> {
    InputData::decode(cursor).map(|x| x.0)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use svm_types::{Address, BytesPrimitive, TemplateAddr};

    use super::*;
    use crate::{test_codec, test_codec_bool};

    #[quickcheck]
    fn encode_decode_bool(b: bool) -> bool {
        test_codec_bool(b)
    }

    #[quickcheck]
    fn encode_decode_u16(n: u16) -> bool {
        test_codec_bool(n)
    }

    #[quickcheck]
    fn encode_decode_u32(n: u32) -> bool {
        test_codec_bool(n)
    }

    #[quickcheck]
    fn encode_decode_u64(n: u64) -> bool {
        test_codec_bool(n)
    }

    #[test]
    fn encode_decode_call() {
        let tx = Transaction {
            version: 0,
            target: Address::of("@target").into(),
            func_name: "do_work".to_string(),
            verifydata: vec![0xAA, 0xBB, 0xCC],
            calldata: vec![0x10, 0x0, 0x30],
        };

        test_codec(tx);
    }

    #[test]
    fn encode_decode_spawn() {
        test_codec(SpawnAccount {
            version: 0,
            account: Account {
                name: "@account".to_string(),
                template_addr: TemplateAddr::of("@template"),
            },
            ctor_name: "initialize".to_string(),
            calldata: vec![0x10, 0x20, 0x30],
        })
    }
}
