use crate::{
    raw::{helpers, NibbleWriter},
    types::WasmValue,
};

use svm_common::Address;

/// Builds a raw representation for `exec-app`
/// Should be used for testing only.
pub struct AppTxBuilder {
    version: Option<u32>,
    app: Option<Address>,
    func_idx: Option<u16>,
    func_buf: Option<Vec<u8>>,
    func_args: Option<Vec<WasmValue>>,
}

#[allow(missing_docs)]
impl AppTxBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            app: None,
            func_idx: None,
            func_buf: None,
            func_args: None,
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_app(mut self, app: &Address) -> Self {
        self.app = Some(app.clone());
        self
    }

    pub fn with_func_index(mut self, func_idx: u16) -> Self {
        self.func_idx = Some(func_idx);
        self
    }

    pub fn with_func_buf(mut self, func_buf: &Vec<u8>) -> Self {
        self.func_buf = Some(func_buf.to_vec());
        self
    }

    pub fn with_func_args(mut self, func_args: &[WasmValue]) -> Self {
        self.func_args = Some(func_args.to_vec());
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut writer = NibbleWriter::new();

        self.write_version(&mut writer);
        self.write_app(&mut writer);
        self.write_func_index(&mut writer);
        self.write_func_buf(&mut writer);
        self.write_func_args(&mut writer);

        helpers::bytes(&mut writer)
    }

    fn write_version(&self, writer: &mut NibbleWriter) {
        let version = self.version.unwrap();

        helpers::encode_version(version, writer);
    }

    fn write_app(&self, writer: &mut NibbleWriter) {
        let addr = self.app.as_ref().unwrap();
        helpers::encode_address(addr, writer);
    }

    fn write_func_index(&mut self, writer: &mut NibbleWriter) {
        let func_idx = self.func_idx.unwrap();

        helpers::encode_varuint14(func_idx, writer);
    }

    fn write_func_buf(&self, writer: &mut NibbleWriter) {
        let buf = if let Some(buf) = &self.func_buf {
            buf.to_vec()
        } else {
            vec![]
        };

        helpers::encode_func_buf(&buf[..], writer)
    }

    fn write_func_args(&self, writer: &mut NibbleWriter) {
        let args = if let Some(args) = &self.func_args {
            args.to_vec()
        } else {
            vec![]
        };

        helpers::encode_func_args(&args[..], writer);
    }
}
