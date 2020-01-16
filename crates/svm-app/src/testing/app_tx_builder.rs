use byteorder::{BigEndian, WriteBytesExt};

use crate::types::{WasmType, WasmValue};

use svm_common::Address;

/// Builds a raw representation for `exec-app`
/// Should be used for testing only.
pub struct AppTxBuilder {
    version: Option<u32>,
    app: Option<Address>,
    func_name: Option<String>,
    func_args: Option<Vec<WasmValue>>,
}

#[allow(missing_docs)]
impl AppTxBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            app: None,
            func_name: None,
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

    pub fn with_func_name(mut self, func_name: &str) -> Self {
        self.func_name = Some(func_name.to_string());
        self
    }

    pub fn with_func_args(mut self, func_args: &[WasmValue]) -> Self {
        self.func_args = Some(func_args.to_vec());
        self
    }

    pub fn build(&mut self) -> Vec<u8> {
        let mut buf = Vec::new();

        self.write_version(&mut buf);
        self.write_app(&mut buf);
        self.write_func_name(&mut buf);
        self.write_func_args(&mut buf);

        buf
    }

    fn write_version(&self, buf: &mut Vec<u8>) {
        let version = self.version.unwrap();
        buf.write_u32::<BigEndian>(version).unwrap();
    }

    fn write_app(&self, buf: &mut Vec<u8>) {
        self.write_address(&self.app.as_ref().unwrap(), buf)
    }

    fn write_func_name(&mut self, buf: &mut Vec<u8>) {
        let name = self.func_name.take().unwrap();
        let bytes = name.as_bytes();

        assert!(bytes.len() <= 255);
        buf.write_u8(bytes.len() as u8).unwrap();

        buf.extend_from_slice(bytes);
    }

    fn write_func_args(&self, buf: &mut Vec<u8>) {
        let args = self.func_args.as_ref().unwrap();

        buf.write_u8(args.len() as u8).unwrap();

        for arg in args {
            match arg {
                WasmValue::I32(v) => {
                    let arg_type = WasmType::I32.into();
                    buf.write_u8(arg_type).unwrap();
                    buf.write_i32::<BigEndian>(*v).unwrap();
                }
                WasmValue::I64(v) => {
                    let arg_type = WasmType::I64.into();
                    buf.write_u8(arg_type).unwrap();
                    buf.write_i64::<BigEndian>(*v).unwrap();
                }
            }
        }
    }

    fn write_address(&self, address: &Address, buf: &mut Vec<u8>) {
        let bytes = address.bytes();
        buf.extend_from_slice(&bytes);
    }
}
