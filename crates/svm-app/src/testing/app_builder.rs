use byteorder::{BigEndian, WriteBytesExt};

use svm_common::Address;

/// Builds a raw representation for `spawn-app`
/// Should be used for testing only.
pub struct AppBuilder {
    version: Option<u32>,
    template: Option<Address>,
    creator: Option<Address>,
}

#[allow(missing_docs)]
impl AppBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            template: None,
            creator: None,
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_template(mut self, template: &Address) -> Self {
        self.template = Some(template.clone());
        self
    }

    pub fn with_creator(mut self, creator: &Address) -> Self {
        self.creator = Some(creator.clone());
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();

        self.write_version(&mut buf);
        self.write_template(&mut buf);
        self.write_creator(&mut buf);

        buf
    }

    fn write_version(&self, buf: &mut Vec<u8>) {
        let version = self.version.unwrap();
        buf.write_u32::<BigEndian>(version).unwrap();
    }

    fn write_template(&self, buf: &mut Vec<u8>) {
        self.write_address(&self.template.as_ref().unwrap(), buf)
    }

    fn write_creator(&self, buf: &mut Vec<u8>) {
        self.write_address(&self.creator.as_ref().unwrap(), buf)
    }

    fn write_address(&self, address: &Address, buf: &mut Vec<u8>) {
        let bytes = address.bytes();
        buf.extend_from_slice(&bytes);
    }
}
