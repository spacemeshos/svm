use std::collections::HashSet;

use svm_types::{
    ApiSection, CodeSection, CtorsSection, DataSection, DeploySection, HeaderSection,
    SchemaSection, Section, SectionKind, Sections,
};

use super::SectionPreview;

use crate::{Codec, ParseError, ReadExt};

/// Decodes a collection of [`Section`] into their native form.
///
/// While running the decoding process, each encountered binary [`Section`] can be decided
/// to be decoded into its matching Rust form or skipped until the next binary [`Section`].
/// This mechanism works thanks to having each binary [`Section`] prefixed with a [`SectionPreview`].
/// It contains information about the kind of [`Section`] and its byte-count.
pub struct SectionsDecoder<'a, T> {
    last_preview: Option<SectionPreview>,
    read_previews: usize,
    section_count: usize,
    reader: &'a mut T,
}

impl<'a, T> SectionsDecoder<'a, T>
where
    T: ReadExt,
{
    /// New Decoder
    pub fn new(reader: &'a mut T) -> Result<Self, ParseError> {
        let mut me = Self {
            reader,
            last_preview: None,
            read_previews: 0,
            section_count: 0,
        };

        me.section_count = me.read_section_count()?;

        Ok(me)
    }

    /// Returns the number of binary [`Section`]s given as input.
    pub fn section_count(&self) -> usize {
        self.section_count
    }

    /// Returns whether decoder has reached it's end of input.
    pub fn is_eof(&mut self) -> bool {
        self.read_previews >= self.section_count
    }

    /// Returns the next [`SectionPreview`].
    pub fn next_preview(&mut self) -> Result<SectionPreview, ParseError> {
        if self.is_eof() {
            return Err(ParseError::Other);
        }

        assert!(
            self.last_preview.is_none(),
            "Please call `decode_section` or `skip_section` prior to calling `next_preview` again"
        );

        let preview = SectionPreview::decode(self.reader)?;

        self.last_preview = Some(preview.clone());
        self.read_previews += 1;

        Ok(preview)
    }

    /// Decodes the current pointed to binary [`Section`].
    pub fn decode_section(&mut self) -> Result<Section, ParseError> {
        assert!(
            self.last_preview.is_some(),
            "Please call `next_preview` prior to calling `decode_section`"
        );

        let last_preview = self.last_preview.take().unwrap();

        let section = match last_preview.kind() {
            SectionKind::Header => HeaderSection::decode(self.reader)?.into(),
            SectionKind::Code => CodeSection::decode(self.reader)?.into(),
            SectionKind::Data => DataSection::decode(self.reader)?.into(),
            SectionKind::Ctors => CtorsSection::decode(self.reader)?.into(),
            SectionKind::Schema => SchemaSection::decode(self.reader)?.into(),
            SectionKind::Api => ApiSection::decode(self.reader)?.into(),
            SectionKind::Deploy => DeploySection::decode(self.reader)?.into(),
        };

        Ok(section)
    }

    /// Skips the current pointed to binary [`Section`].
    pub fn skip_section(&mut self) -> Result<(), ParseError> {
        assert!(
            self.last_preview.is_some(),
            "Please call `next_preview` prior to calling `skip_section`"
        );

        let _bytes = self.section_bytes()?;

        Ok(())
    }

    fn read_section_count(&mut self) -> Result<usize, ParseError> {
        Ok(u16::decode(self.reader)? as usize)
    }

    fn section_bytes(&mut self) -> Result<Vec<u8>, ParseError> {
        let last_preview = self.last_preview.take().unwrap();

        let to_skip = last_preview.byte_size();
        Ok(self.reader.read_bytes(to_skip as usize)?)
    }
}

pub fn decode_sections(
    reader: &mut impl ReadExt,
    interests: Option<HashSet<SectionKind>>,
) -> Result<Sections, ParseError> {
    let mut decoder = SectionsDecoder::new(reader)?;

    let decode_each = interests.is_none();
    let interests = interests.unwrap_or_else(|| HashSet::default());

    let section_count = decoder.section_count();
    let mut sections = Sections::with_capacity(section_count);

    for _ in 0..section_count {
        let preview = decoder.next_preview()?;
        let kind = preview.kind();

        if decode_each || interests.contains(&kind) {
            let section = decoder.decode_section()?;

            sections.insert(section);
        } else {
            decoder.skip_section()?;
        }
    }

    Ok(sections)
}
