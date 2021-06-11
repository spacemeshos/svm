use std::io::Cursor;

use svm_types::SectionKind;

use crate::{Field, ParseError, ReadExt, WriteExt};

pub const CODE_SECTION: u16 = 0x00_01;
pub const DATA_SECTION: u16 = 0x00_02;
pub const CTORS_SECTION: u16 = 0x00_03;
pub const SCHEMA_SECTION: u16 = 0x00_04;
pub const API_SECTION: u16 = 0x00_05;
pub const HEADER_SECTION: u16 = 0x00_06;
pub const DEPLOY_SECTION: u16 = 0x00_07;

pub fn encode(kind: SectionKind, w: &mut Vec<u8>) {
    let raw = match kind {
        SectionKind::Code => CODE_SECTION,
        SectionKind::Data => DATA_SECTION,
        SectionKind::Ctors => CTORS_SECTION,
        SectionKind::Schema => SCHEMA_SECTION,
        SectionKind::Api => API_SECTION,
        SectionKind::Header => HEADER_SECTION,
        SectionKind::Deploy => DEPLOY_SECTION,
    };

    w.write_u16_be(raw);
}

pub fn decode(cursor: &mut std::io::Cursor<&[u8]>) -> Result<SectionKind, ParseError> {
    let value = cursor.read_u16_be();

    if value.is_err() {
        return Err(ParseError::NotEnoughBytes(Field::SectionKind));
    }

    match value.unwrap() {
        CODE_SECTION => Ok(SectionKind::Code),
        DATA_SECTION => Ok(SectionKind::Data),
        CTORS_SECTION => Ok(SectionKind::Ctors),
        SCHEMA_SECTION => Ok(SectionKind::Schema),
        API_SECTION => Ok(SectionKind::Api),
        HEADER_SECTION => Ok(SectionKind::Header),
        DEPLOY_SECTION => Ok(SectionKind::Deploy),
        _ => Err(ParseError::InvalidSection),
    }
}
