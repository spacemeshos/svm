use byteorder::{BigEndian, WriteBytesExt};

use crate::{
    raw::helpers,
    types::{BufferSlice, WasmValue},
};
use svm_common::Address;

/// Builds a raw representation for `spawn-app`
/// Should be used for testing only.
pub struct AppBuilder {
    version: Option<u32>,
    template: Option<Address>,
    ctor_buf: Option<Vec<Vec<u8>>>,
    ctor_args: Option<Vec<WasmValue>>,
}

#[allow(missing_docs)]
impl AppBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            template: None,
            ctor_buf: None,
            ctor_args: None,
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

    pub fn with_ctor_buf(mut self, ctor_buf: &Vec<Vec<u8>>) -> Self {
        self.ctor_buf = Some(ctor_buf.clone());
        self
    }

    pub fn with_ctor_args(mut self, ctor_args: &Vec<WasmValue>) -> Self {
        self.ctor_args = Some(ctor_args.clone());
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();

        self.write_version(&mut buf);
        self.write_template(&mut buf);
        self.write_ctor_buf(&mut buf);
        self.write_ctor_args(&mut buf);

        buf
    }

    fn write_version(&self, buf: &mut Vec<u8>) {
        let version = self.version.unwrap();
        buf.write_u32::<BigEndian>(version).unwrap();
    }

    fn write_template(&self, buf: &mut Vec<u8>) {
        self.write_address(&self.template.as_ref().unwrap(), buf)
    }

    fn write_address(&self, address: &Address, buf: &mut Vec<u8>) {
        let bytes = address.bytes();
        buf.extend_from_slice(&bytes);
    }

    fn write_ctor_buf(&self, buf: &mut Vec<u8>) {
        helpers::write_func_buf(&self.ctor_buf, buf);
    }

    fn write_ctor_args(&self, buf: &mut Vec<u8>) {
        helpers::write_func_args(&self.ctor_args, buf);
    }
}
