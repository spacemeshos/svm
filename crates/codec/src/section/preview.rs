use std::io::Cursor;

use svm_types::SectionKind;

use super::kind;
use crate::{Field, ParseError, ReadExt, WriteExt};

/// Preview data for a [`Section`](svm_types::Section).
#[derive(Debug, Clone, PartialEq)]
pub struct SectionPreview {
    kind: SectionKind,
    byte_size: u32,
}

impl SectionPreview {
    /// New preview.
    pub fn new(kind: SectionKind, byte_size: u32) -> Self {
        Self { kind, byte_size }
    }

    /// Returns the preview's kind.
    pub fn kind(&self) -> SectionKind {
        self.kind
    }

    /// Returns the referred [`Section`](svm_types::Section)'s binary byte size.
    pub fn byte_size(&self) -> u32 {
        self.byte_size
    }

    /// The binary byte size of [`Self`].
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
