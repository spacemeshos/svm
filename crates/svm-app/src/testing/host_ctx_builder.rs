use std::collections::HashMap;

use byteorder::{BigEndian, WriteBytesExt};

pub struct HostCtxBuilder {
    version: Option<u32>,
    fields: HashMap<i32, Vec<u8>>,
}

#[allow(missing_docs)]
impl HostCtxBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            fields: HashMap::new(),
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_raw_field(mut self, idx: i32, value: &[u8]) -> Self {
        self.fields.insert(idx, value.to_owned());
        self
    }

    pub fn with_byte_field(mut self, idx: i32, value: u8) -> Self {
        let mut buf = Vec::with_capacity(1);
        buf.write_u8(value).unwrap();

        self.with_raw_field(idx, &buf[..])
    }

    pub fn with_u16_field(mut self, idx: i32, value: u16) -> Self {
        let mut buf = Vec::with_capacity(2);
        buf.write_u16::<BigEndian>(value).unwrap();

        self.with_raw_field(idx, &buf[..])
    }

    pub fn with_u32_field(mut self, idx: i32, value: u32) -> Self {
        let mut buf = Vec::with_capacity(4);
        buf.write_u32::<BigEndian>(value).unwrap();

        self.with_raw_field(idx, &buf[..])
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();

        self.write_version(&mut buf);
        self.write_fields_count(&mut buf);

        for (idx, vec) in self.fields.iter() {
            self.write_field(*idx, &vec[..], &mut buf);
        }

        buf
    }

    fn write_version(&self, buf: &mut Vec<u8>) {
        let version = self.version.unwrap();
        buf.write_u32::<BigEndian>(version).unwrap();
    }

    fn write_fields_count(&self, buf: &mut Vec<u8>) {
        let fields_count = self.fields.len() as u16;

        buf.write_u16::<BigEndian>(fields_count).unwrap();
    }

    fn write_field(&self, idx: i32, data: &[u8], buf: &mut Vec<u8>) {
        // field `index`
        buf.write_u16::<BigEndian>(idx as u16).unwrap();

        // field `data` length
        buf.write_u16::<BigEndian>(data.len() as u16).unwrap();

        // field `data`
        buf.extend_from_slice(data);
    }
}
