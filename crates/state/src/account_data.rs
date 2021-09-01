use svm_codec::{Codec, ParseError, ReadExt, WriteExt};
use svm_types::{Address, BytesPrimitive, TemplateAddr};

pub struct AccountData {
    pub name: String,
    pub template_addr: TemplateAddr,
}

impl AccountData {
    pub fn key(account_addr: &Address) -> String {
        format!("accounts:{}:immutable", account_addr.to_string())
    }
}

impl Codec for AccountData {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        let encoding_version = 0u8;

        encoding_version.encode(w);
        self.template_addr.encode(w);
        self.name.encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let encoding_version = u8::decode(reader)?;

        if encoding_version != 0 {
            return Err(ParseError::BadByte(encoding_version));
        }

        let template_addr = TemplateAddr::decode(reader)?;
        let name = String::decode(reader)?;

        Ok(Self {
            name,
            template_addr,
        })
    }
}

pub struct AccountMut {
    pub balance: u64,
    pub counter: u64,
}

impl AccountMut {
    pub fn key(account_addr: &Address) -> String {
        format!("accounts:{}:mutable", account_addr.to_string())
    }
}

impl Codec for AccountMut {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        let encoding_version = 0u8;

        encoding_version.encode(w);
        self.balance.encode(w);
        self.counter.encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let encoding_version = u8::decode(reader)?;

        if encoding_version != 0 {
            return Err(ParseError::BadByte(encoding_version));
        }

        let balance = u64::decode(reader)?;
        let counter = u64::decode(reader)?;

        Ok(Self { balance, counter })
    }
}
