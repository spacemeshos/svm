//!
//! # `Code Section`
//!
//! +----------------+----------------+-------------+--------------+--------------+----------+
//! |                |                |             |              |              |          |
//! |   Code Kind    |     Flags      |   Gas Mode  | SVM Version  | Code Length  |   Code   |
//! |   (2 bytes)    |   (8 bytes)    |  (8 bytes)  |  (4 bytes)   |  (4 bytes)   |  (Blob)  |
//! |                |                |             |              |              |          |
//! +----------------+----------------+-------------+--------------+--------------+----------+
//!
//!

use svm_types::{CodeKind, CodeSection, GasMode};

use crate::codec::DataWithPrefix;
use crate::{Codec, ParseError, ReadExt, WriteExt};

pub const WASM: u16 = 0x00_01;
pub const GAS_MODE_FIXED: u64 = 0x00_01;

impl Codec for CodeSection {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        self.kind().encode(w);
        self.flags().encode(w);
        self.gas_mode().encode(w);
        (self.svm_version() as u32).encode(w);
        DataWithPrefix::<u32>::new(self.code().to_vec()).encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        let kind = CodeKind::decode(reader)?;
        let flags = u64::decode(reader)?;
        let gas_mode = GasMode::decode(reader)?;
        let svm_version = u32::decode(reader)?;
        let code = DataWithPrefix::<u32>::decode(reader)?.data;

        Ok(CodeSection::new(kind, code, flags, gas_mode, svm_version))
    }
}

impl Codec for CodeKind {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        match self {
            Self::Wasm => WASM.encode(w),
        }
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match u16::decode(reader)? {
            WASM => Ok(Self::Wasm),
            _ => Err(ParseError::InvalidSection),
        }
    }
}

impl Codec for GasMode {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        match self {
            GasMode::Fixed => GAS_MODE_FIXED.encode(w),
            GasMode::Metering => todo!(),
        }
    }

    fn decode(reader: &mut impl ReadExt) -> Result<Self, Self::Error> {
        match u64::decode(reader)? {
            GAS_MODE_FIXED => Ok(GasMode::Fixed),
            _ => unreachable!(),
        }
    }
}
