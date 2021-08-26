use svm_types::SectionKind;

use crate::{Codec, Field, ParseError, ReadExt};

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

impl Codec for SectionPreview {
    type Error = ParseError;

    fn encode(&self, w: &mut impl crate::WriteExt) {
        self.kind().encode(w);
        self.byte_size().encode(w);
    }

    fn decode(reader: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let kind = SectionKind::decode(reader)?;
        let byte_size =
            u32::decode(reader).map_err(|_| ParseError::Eof(Field::SectionByteSize.to_string()))?;

        Ok(SectionPreview::new(kind, byte_size))
    }
}
