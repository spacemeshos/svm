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

use std::io::Cursor;

use svm_types::{CodeKind, CodeSection, GasMode};

use crate::section::{SectionDecoder, SectionEncoder};
use crate::{Field, ParseError, ReadExt, WriteExt};

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

        w.write_u32_be(length as u32);

        // `Code`
        w.write_bytes(code);
    }
}

impl SectionDecoder for CodeSection {
    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, crate::ParseError> {
        // `Code Kind`
        let kind = decode_code_kind(cursor)?;

        // `Flags`
        let flags = decode_code_flags(cursor)?;

        // `Gas Mode`
        let gas_mode = decode_gas_mode(cursor)?;

        // `SVM Version`
        let svm_version = decode_svm_version(cursor)?;

        // `Code Length`
        match cursor.read_u32_be() {
            Err(..) => Err(ParseError::NotEnoughBytes(Field::Code)),
            Ok(length) => {
                // `Code`

                match cursor.read_bytes(length as usize) {
                    Ok(code) => {
                        let section = CodeSection::new(kind, code, flags, gas_mode, svm_version);

                        Ok(section)
                    }
                    Err(..) => Err(ParseError::NotEnoughBytes(Field::Code)),
                }
            }
        }
    }
}

fn encode_code_kind(kind: CodeKind, w: &mut Vec<u8>) {
    let raw = match kind {
        CodeKind::Wasm => WASM,
    };

    w.write_u16_be(raw);
}

fn decode_code_kind(cursor: &mut Cursor<&[u8]>) -> Result<CodeKind, ParseError> {
    let value = cursor.read_u16_be();

    if value.is_err() {
        return Err(ParseError::NotEnoughBytes(Field::CodeKind));
    }

    match value.unwrap() {
        WASM => Ok(CodeKind::Wasm),
        _ => unreachable!(),
    }
}

fn encode_code_flags(flags: u64, w: &mut Vec<u8>) {
    w.write_u64_be(flags);
}

fn decode_code_flags(cursor: &mut Cursor<&[u8]>) -> Result<u64, ParseError> {
    let value = cursor.read_u64_be();

    value.map_err(|_| ParseError::NotEnoughBytes(Field::CodeFlags))
}

fn encode_gas_mode(gas_mode: GasMode, w: &mut Vec<u8>) {
    match gas_mode {
        GasMode::Fixed => w.write_u64_be(GAS_MODE_FIXED),
        GasMode::Metering => unreachable!(),
    }
}

fn encode_svm_version(svm_ver: u32, w: &mut Vec<u8>) {
    w.write_u32_be(svm_ver);
}

fn decode_gas_mode(cursor: &mut Cursor<&[u8]>) -> Result<GasMode, ParseError> {
    let value = cursor.read_u64_be();

    if value.is_err() {
        return Err(ParseError::NotEnoughBytes(Field::GasMode));
    }

    match value.unwrap() {
        GAS_MODE_FIXED => Ok(GasMode::Fixed),
        _ => unreachable!(),
    }
}

fn decode_svm_version(cursor: &mut Cursor<&[u8]>) -> Result<u32, ParseError> {
    let value = cursor.read_u32_be();

    value.map_err(|_| ParseError::NotEnoughBytes(Field::SvmVersion))
}
