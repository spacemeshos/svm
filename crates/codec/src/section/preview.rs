use std::io::Cursor;

use svm_types::SectionKind;

use crate::{Field, ParseError, ReadExt, WriteExt};

use super::kind;

#[derive(Debug, Clone, PartialEq)]
pub struct SectionPreview {
    kind: SectionKind,

    byte_size: u32,
}

impl SectionPreview {
    pub fn new(kind: SectionKind, byte_size: u32) -> Self {
        Self { kind, byte_size }
    }

    pub fn kind(&self) -> SectionKind {
        self.kind
    }

    pub fn byte_size(&self) -> u32 {
        self.byte_size
    }

    pub const fn len() -> usize {
        8
    }
}

pub fn encode(preview: &SectionPreview, w: &mut Vec<u8>) {
    // `Section Kind`
    kind::encode(preview.kind(), w);

    // `Section Byte Size`
    let byte_size = preview.byte_size();
    w.write_u32_be(byte_size);
}

pub fn decode(cursor: &mut Cursor<&[u8]>) -> Result<SectionPreview, ParseError> {
    // `Section Kind`
    let kind = kind::decode(cursor)?;

    // `Section Byte Size`
    match cursor.read_u32_be() {
        Ok(byte_size) => {
            let preview = SectionPreview::new(kind, byte_size);

            Ok(preview)
        }
        Err(_) => Err(ParseError::NotEnoughBytes(Field::SectionByteSize)),
    }
}
