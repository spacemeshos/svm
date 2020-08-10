use svm_nibble::NibbleWriter;
use svm_types::{AppAddr, AppTransaction, WasmValue};

use crate::api::raw::encode_exec_app;

/// Builds a raw representation for `exec-app`
/// Should be used for testing only.
pub struct AppTxBuilder {
    version: Option<u32>,
    app: Option<AppAddr>,
    func_idx: Option<u16>,
    func_buf: Option<Vec<u8>>,
    func_args: Option<Vec<WasmValue>>,
}

///
/// # Example
///
/// ```rust
/// use svm_types::{AppTransaction, WasmValue, Address};
/// use svm_codec::nibble::NibbleIter;
/// use svm_codec::api::{raw::decode_exec_app, builder::AppTxBuilder};
///
/// let app = Address::of("@my-app").into();
///
/// let func_idx = 10;
/// let func_buf = vec![0x10, 0x20, 0x30];
/// let func_args = vec![WasmValue::I32(40), WasmValue::I64(50)];
///
/// let bytes = AppTxBuilder::new()
///            .with_version(0)
///            .with_app(&app)
///            .with_func_index(func_idx)
///            .with_func_buf(&func_buf)
///            .with_func_args(&func_args[..])
///            .build();
///
/// let mut iter = NibbleIter::new(&bytes[..]);
/// let actual = decode_exec_app(&mut iter).unwrap();
/// let expected = AppTransaction {
///                  version: 0,
///                  app,
///                  func_idx,
///                  func_buf,
///                  func_args,
///                };
///
/// assert_eq!(expected, actual);
/// ```
///
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

    pub fn with_app(mut self, app: &AppAddr) -> Self {
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

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let app = self.app.unwrap();
        let func_idx = self.func_idx.unwrap();

        let func_buf = match self.func_buf {
            None => vec![],
            Some(buf) => buf.to_vec(),
        };

        let func_args = match self.func_args {
            None => vec![],
            Some(args) => args.to_vec(),
        };

        let tx = AppTransaction {
            version,
            app,
            func_idx,
            func_buf,
            func_args,
        };

        let mut w = NibbleWriter::new();

        encode_exec_app(&tx, &mut w);

        w.into_bytes()
    }
}
