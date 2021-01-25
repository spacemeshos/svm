use svm_types::{AppAddr, AppTransaction};

use crate::api::raw::encode_exec_app;

/// Builds a raw representation for `exec-app`
/// Should be used for testing only.
pub struct AppTxBuilder {
    version: Option<u32>,
    app: Option<AppAddr>,
    func_name: Option<String>,
    calldata: Option<Vec<u8>>,
}

///
/// # Example
///
/// ```rust
/// use std::io::Cursor;
///
/// use svm_types::{AppTransaction, Address};
/// use svm_codec::api::{raw::decode_exec_app, builder::AppTxBuilder};
///
/// let app = Address::of("@my-app").into();
///
/// let func_name = "do_work";
/// let calldata = vec![0x10, 0x20, 0x30];
///
/// let bytes = AppTxBuilder::new()
///            .with_version(0)
///            .with_app(&app)
///            .with_func(func_name)
///            .with_calldata(&calldata)
///            .build();
///
/// let mut cursor = Cursor::new(&bytes);
/// let actual = decode_exec_app(&mut cursor).unwrap();
/// let expected = AppTransaction {
///                  version: 0,
///                  app,
///                  func_name: func_name.to_string(),
///                  calldata,
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
            func_name: None,
            calldata: None,
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

    pub fn with_func(mut self, func_name: &str) -> Self {
        self.func_name = Some(func_name.to_string());
        self
    }

    pub fn with_calldata(mut self, calldata: &Vec<u8>) -> Self {
        self.calldata = Some(calldata.to_vec());
        self
    }

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let app = self.app.unwrap();
        let func_name = self.func_name.unwrap();

        let calldata = match self.calldata {
            None => vec![],
            Some(calldata) => calldata.to_vec(),
        };

        let tx = AppTransaction {
            version,
            app,
            func_name,
            calldata,
        };

        let mut w = Vec::new();

        encode_exec_app(&tx, &mut w);

        w
    }
}
