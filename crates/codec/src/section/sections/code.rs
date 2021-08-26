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

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Codec, Field, ParseError, ReadExt, WriteExt};

pub const WASM: u16 = 0x00_01;
pub const GAS_MODE_FIXED: u64 = 0x00_01;

impl SectionEncoder for CodeSection {
    fn encode(&self, w: &mut Vec<u8>) {
        // `Code Kind`
        encode_code_kind(self.kind(), w);

        // `Flags`
        encode_code_flags(self.flags(), w);

        // `Gas Mode`
        encode_gas_mode(self.gas_mode(), w);

        // `SVM Version`
        encode_svm_version(self.svm_version(), w);

        // `Code Length`
        let code = self.code();
        let length = code.len();
        assert!(length < std::u32::MAX as usize);

        (length as u32).encode(w);

        // `Code`
        w.write_bytes(code);
    }
}

impl SectionDecoder for CodeSection {
    fn decode(cursor: &mut impl ReadExt) -> Result<Self, crate::ParseError> {
        // `Code Kind`
        let kind = decode_code_kind(cursor)?;

        // `Flags`
        let flags = decode_code_flags(cursor)?;

        // `Gas Mode`
        let gas_mode = decode_gas_mode(cursor)?;

        // `SVM Version`
        let svm_version = decode_svm_version(cursor)?;

        // `Code Length`
        match u32::decode(cursor) {
            Err(..) => Err(ParseError::Eof(Field::Code.to_string())),
            Ok(length) => {
                // `Code`

                match cursor.read_bytes(length as usize) {
                    Ok(code) => {
                        let section = CodeSection::new(kind, code, flags, gas_mode, svm_version);

                        Ok(section)
                    }
                    Err(..) => Err(ParseError::Eof(Field::Code.to_string())),
                }
            }
        }
    }
}

fn encode_code_kind(kind: CodeKind, w: &mut Vec<u8>) {
    let raw = match kind {
        CodeKind::Wasm => WASM,
    };

    (raw as u16).encode(w);
}

fn decode_code_kind(cursor: &mut impl ReadExt) -> Result<CodeKind, ParseError> {
    let value = u16::decode(cursor);

    if value.is_err() {
        return Err(ParseError::Eof(Field::CodeKind.to_string()));
    }

    match value.unwrap() {
        WASM => Ok(CodeKind::Wasm),
        _ => unreachable!(),
    }
}

fn encode_code_flags(flags: u64, w: &mut Vec<u8>) {
    flags.encode(w);
}

fn decode_code_flags(cursor: &mut impl ReadExt) -> Result<u64, ParseError> {
    let value = u64::decode(cursor);

    value.map_err(|_| ParseError::Eof(Field::CodeFlags.to_string()))
}

fn encode_gas_mode(gas_mode: GasMode, w: &mut Vec<u8>) {
    match gas_mode {
        GasMode::Fixed => GAS_MODE_FIXED.encode(w),
        GasMode::Metering => todo!(),
    }
}

fn encode_svm_version(svm_ver: u32, w: &mut Vec<u8>) {
    svm_ver.encode(w);
}

fn decode_gas_mode(cursor: &mut impl ReadExt) -> Result<GasMode, ParseError> {
    let value = u64::decode(cursor);

    if value.is_err() {
        return Err(ParseError::Eof(Field::GasMode.to_string()));
    }

    match value.unwrap() {
        GAS_MODE_FIXED => Ok(GasMode::Fixed),
        _ => unreachable!(),
    }
}

fn decode_svm_version(cursor: &mut impl ReadExt) -> Result<u32, ParseError> {
    let value = u32::decode(cursor);

    value.map_err(|_| ParseError::Eof(Field::SvmVersion.to_string()))
}
