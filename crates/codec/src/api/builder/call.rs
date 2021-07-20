use svm_types::{AccountAddr, Transaction};

use crate::call;

/// Builds a raw representation for [`Transaction`]
///
/// Should be used mainly for testing only.
pub struct TxBuilder {
    version: Option<u16>,
    target: Option<AccountAddr>,
    func_name: Option<String>,
    // verifydata: Option<Vec<u8>>,
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
/// use svm_codec::call;
///
/// let target = Address::of("@target").into();
///
/// let func_name = "do_work";
/// // let verifydata = vec![0x10, 0x20, 0x30];
/// let calldata = vec![0x10, 0x20, 0x30];
///
/// let bytes = TxBuilder::new()
///            .with_version(0)
///            .with_target(&target)
///            .with_func(func_name)
///            // .with_verifydata(&verifydata)
///            .with_calldata(&calldata)
///            .build();
///
/// let mut cursor = Cursor::new(&bytes[..]);
/// let actual = call::decode_call(&mut cursor).unwrap();
/// let expected = Transaction {
///                  version: 0,
///                  target,
///                  func_name: func_name.to_string(),
///                  // verifydata,
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
            target: None,
            func_name: None,
            // verifydata: None,
            calldata: None,
        }
    }

    pub fn with_version(mut self, version: u16) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_target(mut self, target: &AccountAddr) -> Self {
        self.target = Some(target.clone());
        self
    }

    pub fn with_func(mut self, func_name: &str) -> Self {
        self.func_name = Some(func_name.to_string());
        self
    }

    // pub fn with_verifydata(mut self, verifydata: &[u8]) -> Self {
    //     self.verifydata = Some(verifydata.to_vec());
    //     self
    // }

    pub fn with_calldata(mut self, calldata: &[u8]) -> Self {
        self.calldata = Some(calldata.to_vec());
        self
    }

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let target = self.target.unwrap();
        let func_name = self.func_name.unwrap();

        // let verifydata = match self.verifydata {
        //     None => vec![],
        //     Some(verifydata) => verifydata.to_vec(),
        // };

        let calldata = match self.calldata {
            None => vec![],
            Some(calldata) => calldata.to_vec(),
        };

        let tx = Transaction {
            version,
            target,
            function: func_name,
            // verifydata,
            calldata,
        };

        let mut w = Vec::new();

        call::encode_call(&tx, &mut w);

        w
    }
}
