use svm_types::{AppAddr, Transaction};

use crate::transaction;

/// Builds a raw representation for `exec-app`
/// Should be used for testing only.
pub struct TxBuilder {
    version: Option<u16>,
    app: Option<AppAddr>,
    func_name: Option<String>,
    verifydata: Option<Vec<u8>>,
    calldata: Option<Vec<u8>>,
}

///
/// # Example
///
/// ```rust
/// use std::io::Cursor;
///
/// use svm_types::{Transaction, Address};
/// use svm_codec::api::builder::TxBuilder;
/// use svm_codec::transaction;
///
/// let app = Address::of("@my-app").into();
///
/// let func_name = "do_work";
/// let verifydata = vec![0x10, 0x20, 0x30];
/// let calldata = vec![0x10, 0x20, 0x30];
///
/// let bytes = TxBuilder::new()
///            .with_version(0)
///            .with_app(&app)
///            .with_func(func_name)
///            .with_verifydata(&verifydata)
///            .with_calldata(&calldata)
///            .build();
///
/// let mut cursor = Cursor::new(&bytes[..]);
/// let actual = transaction::decode_exec_app(&mut cursor).unwrap();
/// let expected = Transaction {
///                  version: 0,
///                  app,
///                  func_name: func_name.to_string(),
///                  verifydata,
///                  calldata,
///                };
///
/// assert_eq!(expected, actual);
/// ```
///
#[allow(missing_docs)]
impl TxBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            app: None,
            func_name: None,
            verifydata: None,
            calldata: None,
        }
    }

    pub fn with_version(mut self, version: u16) -> Self {
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

    pub fn with_verifydata(mut self, verifydata: &[u8]) -> Self {
        self.verifydata = Some(verifydata.to_vec());
        self
    }

    pub fn with_calldata(mut self, calldata: &[u8]) -> Self {
        self.calldata = Some(calldata.to_vec());
        self
    }

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let app = self.app.unwrap();
        let func_name = self.func_name.unwrap();

        let verifydata = match self.verifydata {
            None => vec![],
            Some(verifydata) => verifydata.to_vec(),
        };

        let calldata = match self.calldata {
            None => vec![],
            Some(calldata) => calldata.to_vec(),
        };

        let tx = Transaction {
            version,
            app,
            func_name,
            verifydata,
            calldata,
        };

        let mut w = Vec::new();

        transaction::encode_exec_app(&tx, &mut w);

        w
    }
}
