use byteorder::{BigEndian, WriteBytesExt};

use svm_common::Address;

/// A raw `AppTemplate` builder. Used for testing.
pub struct WireAppTemplateBuilder {
    version: Option<u32>,
    name: Option<String>,
    author: Option<Address>,
    code: Option<Vec<u8>>,
}

#[allow(missing_docs)]
impl WireAppTemplateBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            name: None,
            author: None,
            code: None,
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_author(mut self, address: Address) -> Self {
        self.author = Some(address);
        self
    }

    pub fn with_code(mut self, code: &[u8]) -> Self {
        self.code = Some(code.to_vec());
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();

        self.write_version(&mut buf);
        self.write_name(&mut buf);
        self.write_author(&mut buf);
        self.write_admins(&mut buf);
        self.write_deps(&mut buf);
        self.write_code(&mut buf);

        buf
    }

    fn write_version(&self, buf: &mut Vec<u8>) {
        let version = self.version.unwrap();
        buf.write_u32::<BigEndian>(version).unwrap();
    }

    fn write_name(&mut self, buf: &mut Vec<u8>) {
        let name = self.name.take().unwrap();
        let bytes = name.as_bytes();

        assert!(bytes.len() <= 255);
        buf.write_u8(bytes.len() as u8).unwrap();

        buf.extend_from_slice(bytes);
    }

    fn write_author(&self, buf: &mut Vec<u8>) {
        let author = self.author.as_ref().unwrap();
        buf.extend_from_slice(author.as_slice());
    }

    fn write_admins(&self, buf: &mut Vec<u8>) {
        buf.write_u16::<BigEndian>(0).unwrap();
    }

    fn write_deps(&self, buf: &mut Vec<u8>) {
        buf.write_u16::<BigEndian>(0).unwrap();
    }

    fn write_code(&self, buf: &mut Vec<u8>) {
        let code = self.code.as_ref().unwrap();

        buf.write_u64::<BigEndian>(code.len() as u64).unwrap();
        buf.extend_from_slice(code.as_slice());
    }
}
