use std::io::Cursor;

use svm_types::{
    Account, Address, Context, Envelope, Gas, Layer, SpawnAccount, TemplateAddr, Transaction,
};

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
    type Error = std::io::Error;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_address(self.principal());
        w.write_u64_be(self.amount());
        w.write_u64_be(self.gas_limit().unwrap_or(0));
        w.write_u64_be(self.gas_fee());
    }

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let principal = cursor.read_address()?;
        let amount = cursor.read_u64_be()?;
        let gas_limit = cursor.read_u64_be()?;
        let gas_fee = cursor.read_u64_be()?;

        let gas_limit = if gas_limit > 0 {
            Gas::with(gas_limit)
        } else {
            Gas::new()
        };

        let envelope = Envelope::new(principal, amount, gas_limit, gas_fee);
        Ok(envelope)
    }

    fn fixed_size() -> Option<usize> {
        Some(20 + 8 + 8 + 8)
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
        version::encode_version(self.version, w);
        w.write_address(self.target());
        w.write_string(self.func_name());
        InputData::encode(&InputData(self.verifydata.clone()), w);
        InputData::encode(&InputData(self.calldata.clone()), w);
    }

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let version = decode_version(cursor)?;
        let target = decode_target(cursor)?;
        let func_name = decode_func(cursor)?;
        let verifydata = InputData::decode(cursor)?.0.to_vec();
        let calldata = InputData::decode(cursor)?.0.to_vec();

        let tx = Transaction {
            version,
            target,
            func_name,
            verifydata,
            calldata,
        };

        Ok(tx)
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
    type Error = std::io::Error;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_tx_id(self.tx_id());
        w.write_u64_be(self.layer().0);
        w.write_state(self.state());
    }

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let tx_id = cursor.read_tx_id()?;
        let layer = cursor.read_u64_be()?;
        let state = cursor.read_state()?;

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
        version::encode_version(self.version, w);
        w.write_template_addr(self.template_addr());
        w.write_string(self.account_name());
        w.write_string(self.ctor_name());
        InputData(self.calldata.clone()).encode(w);
    }

    fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<Self, Self::Error> {
        let version = decode_version(cursor)?;
        let template_addr = decode_template(cursor)?;
        let name = decode_name(cursor)?;
        let ctor_name = decode_ctor(cursor)?;
        let calldata = decode_ctor_calldata(cursor)?;

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

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        match cursor.read_byte() {
            Err(..) => Err(ParseError::NotEnoughBytes(Field::InputDataLength)),
            Ok(byte) => {
                let length = byte as usize;

                cursor
                    .read_bytes(length)
                    .map(|x| Self(x))
                    .map_err(|_| ParseError::NotEnoughBytes(Field::InputData))
            }
        }
    }
}

/// Decoders

#[inline]
fn decode_version(cursor: &mut Cursor<&[u8]>) -> Result<u16, ParseError> {
    version::decode_version(cursor)
}

fn decode_template(cursor: &mut Cursor<&[u8]>) -> Result<TemplateAddr, ParseError> {
    cursor
        .read_template_addr()
        .map_err(|_| ParseError::NotEnoughBytes(Field::Address))
}

fn decode_name(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(name)) => Ok(name),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Name)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Name)),
    }
}

fn decode_ctor(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(ctor)) => Ok(ctor),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Ctor)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Ctor)),
    }
}

fn decode_ctor_calldata(cursor: &mut Cursor<&[u8]>) -> Result<Vec<u8>, ParseError> {
    InputData::decode(cursor).map(|x| x.0)
}

fn decode_target(cursor: &mut Cursor<&[u8]>) -> Result<Address, ParseError> {
    cursor
        .read_address()
        .map_err(|_| ParseError::NotEnoughBytes(Field::TargetAddr))
}

fn decode_func(cursor: &mut Cursor<&[u8]>) -> Result<String, ParseError> {
    match cursor.read_string() {
        Ok(Ok(func)) => Ok(func),
        Ok(Err(..)) => Err(ParseError::InvalidUTF8String(Field::Function)),
        Err(..) => Err(ParseError::NotEnoughBytes(Field::Function)),
    }
}

#[cfg(test)]
mod tests {
    use svm_types::{Address, BytesPrimitive, TemplateAddr};

    use super::*;
    use crate::test_codec;

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
