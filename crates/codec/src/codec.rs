use std::convert::{TryFrom, TryInto};
use std::io::Cursor;
use std::marker::PhantomData;

use svm_types::{
    Account, Address, Context, Envelope, Gas, Layer, SectionKind, SpawnAccount, State,
    TemplateAddr, Transaction, TransactionId,
};

use crate::error::{BoolError, EofError, StringError};
use crate::{ParseError, ReadExt, WriteExt};

/// Ability to encode and decode items of a certain type.
pub trait Codec: Sized {
    /// The type of errors that can arise during decoding operations.
    ///
    /// This should be [`std::convert::Infallible`] if nonexistant.
    type Error;

    /// Writes a binary representation of `self` to `w`.
    ///
    /// # Panics
    ///
    /// This method does not return any errors. Instead, it panics if `self`
    /// does not meet the conditions necessary for encoding (e.g. a max. size).
    fn encode(&self, w: &mut impl WriteExt);

    /// Attempts to parse a binary representation of `Self` pointed at by
    /// `cursor`. Returns a [`Codec::Error`] on failure.
    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error>;

    /// Like [`Codec::decode`], but can be used with anything resembling bytes.
    fn decode_bytes<B>(bytes: B) -> Result<Self, Self::Error>
    where
        B: AsRef<[u8]>,
    {
        Self::decode(&mut Cursor::new(bytes.as_ref()))
    }

    /// In case `Self` has a binary representation with a fixed size, this
    /// should return [`Some`] with the appropriate size; [`None`] otherwise. It
    /// can be used for pre-allocation optimizations.
    fn fixed_size() -> Option<usize> {
        None
    }

    /// Calls [`Codec::encode`] with an empty [`Vec<u8>`] and immediately
    /// returns it.
    fn encode_to_vec(&self) -> Vec<u8> {
        let mut w = Vec::with_capacity(Self::fixed_size().unwrap_or_default());
        self.encode(&mut w);
        w
    }
}

#[cfg(test)]
pub fn test_codec<T, E>(item: T)
where
    T: Codec<Error = E> + std::fmt::Debug + PartialEq,
    E: std::fmt::Debug,
{
    let encoded = item.encode_to_vec();
    let decoded = T::decode_bytes(&encoded).unwrap();

    assert_eq!(item, decoded);
    if let Some(fixed_size) = T::fixed_size() {
        assert_eq!(fixed_size, encoded.len());
    }
}

#[cfg(test)]
pub fn test_codec_bool<T, E>(item: T) -> bool
where
    T: Codec<Error = E> + std::fmt::Debug + PartialEq,
    E: std::fmt::Debug,
{
    let encoded = item.encode_to_vec();
    let decoded = T::decode_bytes(&encoded).unwrap();

    item == decoded && T::fixed_size().map(|x| x == encoded.len()).unwrap_or(true)
}

/// A sequence of bytes prefixed by their length in the form of a numeric type
/// `T`.
#[derive(Debug)]
pub struct DataWithPrefix<P> {
    pub data: Vec<u8>,
    phantom: PhantomData<P>,
}

pub type InputData = DataWithPrefix<u8>;
pub type ReturnData = DataWithPrefix<u16>;

impl<P> DataWithPrefix<P> {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            phantom: PhantomData::default(),
        }
    }
}

impl<P> Codec for DataWithPrefix<P>
where
    P: Into<u64> + TryFrom<u64> + Codec,
{
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        P::try_from(self.data.len() as u64)
            .map_err(|_| ())
            .unwrap()
            .encode(w);
        w.write_bytes(&self.data);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, ParseError> {
        let len = P::decode(reader).map_err(|_| ParseError::Eof)?;
        let len: u64 = len.into();

        Ok(Self::new(reader.read_bytes(len as usize)?))
    }
}

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

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let principal = Address::decode(reader)?;
        let amount = u64::decode(reader)?;
        let gas_limit = Gas::decode(reader)?;
        let gas_fee = u64::decode(reader)?;

        let envelope = Envelope::new(principal, amount, gas_limit, gas_fee);
        Ok(envelope)
    }

    fn fixed_size() -> Option<usize> {
        Some(20 + 8 + 8 + 8)
    }
}

impl Codec for Gas {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.unwrap_or(0).encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match u64::decode(reader)? {
            0 => Ok(Gas::new()),
            x => Ok(Gas::with(x)),
        }
    }
}

impl<const N: usize> Codec for [u8; N] {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_bytes(&self[..])
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        Ok((&reader.read_bytes(N)?[..]).try_into().unwrap())
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
        (self.version as u16).encode(w);
        self.target().encode(w);
        self.func_name().to_string().encode(w);
        InputData::new(self.verifydata.clone()).encode(w);
        InputData::new(self.calldata.clone()).encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let version = u16::decode(reader)?;
        let target = Address::decode(reader)?.into();
        let func_name = String::decode(reader)?;
        let verifydata = InputData::decode(reader)?.data;
        let calldata = InputData::decode(reader)?.data;

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
        self.tx_id().encode(w);
        self.layer().0.encode(w);
        self.state().encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let tx_id = TransactionId::decode(reader)?;
        let layer = u64::decode(reader)?;
        let state = State::decode(reader)?;

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
        u16::try_from(self.version).unwrap().encode(w);
        self.template_addr().0.encode(w);
        self.account_name().to_string().encode(w);
        self.ctor_name().to_string().encode(w);
        InputData::new(self.calldata.clone()).encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let version = u16::decode(reader)?;
        let template_addr = TemplateAddr::decode(reader)?.into();
        let name = String::decode(reader)?;
        let ctor_name = String::decode(reader)?;
        let calldata = InputData::decode(reader)?.data;

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

impl Codec for String {
    type Error = StringError;

    fn encode(&self, w: &mut impl WriteExt) {
        let len =
            u8::try_from(self.as_bytes().len()).expect("The string is too long, max 255 bytes.");
        w.write_byte(len);
        w.write_bytes(self.as_bytes());
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let len = reader.read_byte()? as usize;
        let bytes = reader.read_bytes(len)?;

        Ok(String::from_utf8(bytes.to_vec())?)
    }
}

impl Codec for u8 {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(*self);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        reader.read_byte()
    }
}

impl Codec for u16 {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_bytes(&self.to_be_bytes());
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 2];
        reader.read_fill(&mut buf[..])?;
        Ok(u16::from_be_bytes(buf))
    }
}

impl Codec for u32 {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_bytes(&self.to_be_bytes());
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 4];
        reader.read_fill(&mut buf[..])?;
        Ok(u32::from_be_bytes(buf))
    }
}

impl Codec for u64 {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_bytes(&self.to_be_bytes());
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let mut buf = [0u8; 8];
        reader.read_fill(&mut buf[..])?;
        Ok(u64::from_be_bytes(buf))
    }
}

impl Codec for bool {
    type Error = BoolError;

    fn encode(&self, w: &mut impl WriteExt) {
        w.write_byte(if *self { 1 } else { 0 });
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let byte = reader.read_byte()?;
        match byte {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(BoolError::InvalidByte(byte)),
        }
    }
}

impl<T> Codec for Vec<T>
where
    T: Codec<Error = ParseError>,
{
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        assert!(self.len() <= u16::MAX as usize);
        (self.len() as u16).encode(w);

        for elem in self {
            elem.encode(w);
        }
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let len = u16::decode(reader)?;
        let mut vec = Vec::with_capacity(len as usize);

        for _ in 0..len {
            vec.push(T::decode(reader)?);
        }

        Ok(vec)
    }
}

const SECTION_KIND_CODE_SECTION: u16 = 1;
const SECTION_KIND_DATA_SECTION: u16 = 2;
const SECTION_KIND_CTORS_SECTION: u16 = 3;
const SECTION_KIND_SCHEMA_SECTION: u16 = 4;
const SECTION_KIND_API_SECTION: u16 = 5;
const SECTION_KIND_HEADER_SECTION: u16 = 6;
const SECTION_KIND_DEPLOY_SECTION: u16 = 7;

impl Codec for SectionKind {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        match self {
            Self::Code => SECTION_KIND_CODE_SECTION,
            Self::Data => SECTION_KIND_DATA_SECTION,
            Self::Ctors => SECTION_KIND_CTORS_SECTION,
            Self::Schema => SECTION_KIND_SCHEMA_SECTION,
            Self::Api => SECTION_KIND_API_SECTION,
            Self::Header => SECTION_KIND_HEADER_SECTION,
            Self::Deploy => SECTION_KIND_DEPLOY_SECTION,
        }
        .encode(w)
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match u16::decode(reader)? {
            SECTION_KIND_CODE_SECTION => Ok(Self::Code),
            SECTION_KIND_DATA_SECTION => Ok(Self::Data),
            SECTION_KIND_CTORS_SECTION => Ok(Self::Ctors),
            SECTION_KIND_SCHEMA_SECTION => Ok(Self::Schema),
            SECTION_KIND_API_SECTION => Ok(Self::Api),
            SECTION_KIND_HEADER_SECTION => Ok(Self::Header),
            SECTION_KIND_DEPLOY_SECTION => Ok(Self::Deploy),
            _ => Err(Self::Error::InvalidSection),
        }
    }
}

impl Codec for TemplateAddr {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.0.encode(w);
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        Ok(<[u8; 20]>::decode(cursor)?.into())
    }
}

impl Codec for State {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.0.encode(w);
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        Ok(<[u8; 32]>::decode(cursor)?.into())
    }
}

impl Codec for Address {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.0.encode(w);
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        Ok(<[u8; 20]>::decode(cursor)?.into())
    }
}

impl Codec for TransactionId {
    type Error = EofError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.0.encode(w);
    }

    fn decode(cursor: &mut impl ReadExt) -> Result<Self, Self::Error> {
        Ok(<[u8; 32]>::decode(cursor)?.into())
    }
}

/// Decoders

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use svm_types::{Address, BytesPrimitive, TemplateAddr};

    use super::*;

    #[quickcheck]
    fn encode_then_decode_bool(b: bool) -> bool {
        test_codec_bool(b)
    }

    #[quickcheck]
    fn encode_then_decode_u16(n: u16) -> bool {
        test_codec_bool(n)
    }

    #[quickcheck]
    fn encode_then_decode_u32(n: u32) -> bool {
        test_codec_bool(n)
    }

    #[quickcheck]
    fn encode_then_decode_u64(n: u64) -> bool {
        test_codec_bool(n)
    }

    #[test]
    #[should_panic]
    fn encode_long_string_panics() {
        let s = std::iter::repeat('0').take(1000).collect::<String>();
        s.encode_to_vec();
    }

    #[test]
    fn encode_then_decode_string() {
        test_codec("Hello".to_string());
        test_codec("Bonjour".to_string());
        test_codec("¿Qué tal?".to_string());
        test_codec("0000000000000000000000000000".to_string());
    }

    #[test]
    fn encode_then_decode_byte_primitives() {
        test_codec(Address::zeros());
        test_codec(TemplateAddr::zeros());
        test_codec(State::zeros());
        test_codec(TransactionId::zeros());
    }

    #[test]
    fn encode_then_decode_section_kind() {
        test_codec(SectionKind::Api);
        test_codec(SectionKind::Code);
        test_codec(SectionKind::Schema);
    }

    #[test]
    fn invalid_section_kind() {
        assert!(SectionKind::decode_bytes(&u32::MAX.to_be_bytes()).is_err());
    }

    #[test]
    fn encode_then_decode_transaction() {
        test_codec(Transaction {
            version: 0,
            target: Address::of("@target").into(),
            func_name: "do_work".to_string(),
            verifydata: vec![0xAA, 0xBB, 0xCC],
            calldata: vec![0x10, 0x0, 0x30],
        });
        test_codec(Transaction {
            version: u16::MAX,
            target: Address::of("FOOBAR").into(),
            func_name: "work".to_string(),
            verifydata: vec![0xAA, 0xBB, 0xCC],
            calldata: vec![0x10, 0x0, 0x30],
        });
    }

    #[test]
    fn encode_then_decode_spawn() {
        test_codec(SpawnAccount {
            version: 0,
            account: Account {
                name: "@account".to_string(),
                template_addr: TemplateAddr::of("@template"),
            },
            ctor_name: "initialize".to_string(),
            calldata: vec![0x10, 0x20, 0x30],
        });
        test_codec(SpawnAccount {
            version: u16::MAX,
            account: Account {
                name: "@foobar".to_string(),
                template_addr: TemplateAddr::of("@spam"),
            },
            ctor_name: "work!".to_string(),
            calldata: vec![0x10, 0x20, 0x30],
        });
    }

    #[quickcheck]
    fn encode_then_decode_context(layer: u64) -> bool {
        test_codec_bool(Context::new(
            TransactionId::zeros(),
            Layer(layer),
            State::zeros(),
        ))
    }

    #[quickcheck]
    fn encode_then_decode_envelope(amount: u64, gas_limit: u64, gas_fee: u64) -> bool {
        test_codec_bool(Envelope::new(
            Address::zeros(),
            amount,
            if gas_limit == 0 {
                Gas::new()
            } else {
                Gas::from(gas_limit)
            },
            gas_fee,
        ))
    }
}
