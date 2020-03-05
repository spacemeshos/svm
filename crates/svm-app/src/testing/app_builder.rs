use crate::{
    raw::{helpers, NibbleWriter},
    types::WasmValue,
};

use svm_common::Address;

/// Builds a raw representation for `spawn-app`
/// Should be used for testing only.
pub struct SpawnAppBuilder {
    version: Option<u32>,
    template: Option<Address>,
    ctor_idx: Option<u16>,
    ctor_buf: Option<Vec<u8>>,
    ctor_args: Option<Vec<WasmValue>>,
}

///
/// # Example
///
/// ```rust
/// use svm_app::{testing::SpawnAppBuilder, types::{App, SpawnApp, WasmValue}, raw::parse_app};
/// use svm_common::Address;
////
/// let template = Address::of("@template");
/// let creator = Address::of("@creator");
/// let ctor_idx = 2;
/// let ctor_buf = vec![0x10, 0x20, 0x30];
/// let ctor_args = vec![WasmValue::I32(0x40), WasmValue::I64(0x50)];
///
/// let bytes = SpawnAppBuilder::new()
///  .with_version(0)
///  .with_template(&template)
///  .with_ctor_index(ctor_idx)
///  .with_ctor_buf(&ctor_buf)
///  .with_ctor_args(&ctor_args)
///  .build();
///
/// let actual = parse_app(&bytes[..], &creator).unwrap();
/// let expected = SpawnApp {
///                  app: App { template, creator },
///                  ctor_idx,
///                  ctor_buf,
///                  ctor_args
///                };
///
//// assert_eq!(expected, actual);
/// ```
///

#[allow(missing_docs)]
impl SpawnAppBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            template: None,
            ctor_idx: None,
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

    pub fn with_ctor_index(mut self, ctor_idx: u16) -> Self {
        self.ctor_idx = Some(ctor_idx);
        self
    }

    pub fn with_ctor_buf(mut self, ctor_buf: &Vec<u8>) -> Self {
        self.ctor_buf = Some(ctor_buf.clone());
        self
    }

    pub fn with_ctor_args(mut self, ctor_args: &Vec<WasmValue>) -> Self {
        self.ctor_args = Some(ctor_args.clone());
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut writer = NibbleWriter::new();

        self.write_version(&mut writer);
        self.write_template(&mut writer);
        self.write_ctor_index(&mut writer);
        self.write_ctor_buf(&mut writer);
        self.write_ctor_args(&mut writer);

        helpers::bytes(&mut writer)
    }

    fn write_version(&self, writer: &mut NibbleWriter) {
        let version = self.version.unwrap();

        helpers::encode_version(version, writer);
    }

    fn write_template(&self, writer: &mut NibbleWriter) {
        let addr = self.template.as_ref().unwrap();

        helpers::encode_address(addr, writer);
    }

    fn write_ctor_index(&self, writer: &mut NibbleWriter) {
        let ctor_idx = self.ctor_idx.unwrap();

        helpers::encode_varuint14(ctor_idx, writer);
    }

    fn write_ctor_buf(&self, writer: &mut NibbleWriter) {
        let buf = if let Some(buf) = &self.ctor_buf {
            buf.to_vec()
        } else {
            vec![]
        };

        helpers::encode_func_buf(&buf[..], writer);
    }

    fn write_ctor_args(&self, writer: &mut NibbleWriter) {
        let args = if let Some(args) = &self.ctor_args {
            args.to_vec()
        } else {
            vec![]
        };

        helpers::encode_func_args(&args[..], writer);
    }
}
