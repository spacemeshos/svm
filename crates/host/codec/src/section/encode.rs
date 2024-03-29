use indexmap::IndexMap;

use std::u16;

use svm_types::{Section, SectionKind, Sections};

use super::SectionPreview;
use crate::Codec;
use crate::WriteExt;

/// Encodes a collection of [`Section`] into a binary form.
pub struct SectionsEncoder {
    section_buf: IndexMap<SectionKind, Vec<u8>>,
}

impl Default for SectionsEncoder {
    fn default() -> Self {
        Self {
            section_buf: IndexMap::with_capacity(0),
        }
    }
}

impl SectionsEncoder {
    /// Creates a new encoder,and allocates room for `capacity` sections.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            section_buf: IndexMap::with_capacity(capacity),
        }
    }

    /// Encodes each [`Section`] provided by `sections` and stores them internally.
    pub fn encode(&mut self, sections: &Sections) {
        for section in sections.iter() {
            self.encode_section(section);
        }
    }

    /// Returns the binary encoding of the [`Section`]s provided so far.
    pub fn finish(mut self) -> Vec<u8> {
        let section_count = self.section_buf.len();
        assert!(section_count < std::u16::MAX as usize);

        let section_count_size = 2;
        let previews_size = section_count * SectionPreview::fixed_size().unwrap();
        let sections_size: usize = self.section_buf.values().map(|buf| buf.len()).sum();

        let capacity = section_count_size + previews_size + sections_size;
        let mut w = Vec::with_capacity(capacity);

        // Section Count
        (section_count as u16).encode(&mut w);

        for (kind, bytes) in self.section_buf.drain(..) {
            // Section Preview
            let byte_size = bytes.len();
            assert!(byte_size < std::u32::MAX as usize);

            let preview = SectionPreview::new(kind, byte_size as u32);
            preview.encode(&mut w);

            // `Section`
            w.write_bytes(&bytes);
        }

        w
    }

    fn encode_section(&mut self, section: &Section) {
        let kind = section.kind();
        let buf = self.section_buf_mut(kind);

        match kind {
            SectionKind::Api => section.as_api().encode(buf),
            SectionKind::Header => section.as_header().encode(buf),
            SectionKind::Code => section.as_code().encode(buf),
            SectionKind::Data => section.as_data().encode(buf),
            SectionKind::Ctors => section.as_ctors().encode(buf),
            SectionKind::Schema => section.as_schema().encode(buf),
            SectionKind::Deploy => section.as_deploy().encode(buf),
        }
    }

    fn section_buf_mut(&mut self, kind: SectionKind) -> &mut Vec<u8> {
        // initializes a `Section buffer` if not exists
        let _entry = self.section_buf.entry(kind).or_insert_with(|| Vec::new());

        if let Some(buf) = self.section_buf.get_mut(&kind) {
            buf
        } else {
            unreachable!()
        }
    }
}
