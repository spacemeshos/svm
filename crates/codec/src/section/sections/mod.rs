mod api;
mod code;
mod ctors;
mod data;
mod deploy;
mod header;
mod schema;

pub trait SectionEncoder: Sized {
    fn encode(&self, w: &mut Vec<u8>);

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError>;
}

use crate::ParseError;

use std::io::Cursor;
use svm_types::{Section, Sections};

impl SectionEncoder for Section {
    fn encode(&self, w: &mut Vec<u8>) {
        match self {
            Section::Api(api) => api.encode(w),
            _ => todo!(),
        }
    }

    fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Self, ParseError> {
        todo!()
    }
}

pub struct SectionsEncoder {
    buf: Vec<u8>,
}

impl SectionsEncoder {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn encode(&mut self, sections: Sections) {
        for section in sections.iter() {
            section.encode(&mut self.buf);
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}

pub struct SectionsDecoder<'a> {
    cursor: Cursor<&'a [u8]>,
}

impl<'a> SectionsDecoder<'a> {
    pub fn new(cursor: Cursor<&'a [u8]>) -> Self {
        Self { cursor }
    }

    pub fn peek(&mut self) {
        todo!()
    }

    pub fn skip(&mut self) {
        todo!()
    }
}
