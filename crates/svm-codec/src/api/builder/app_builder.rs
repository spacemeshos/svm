use svm_nibble::NibbleWriter;
use svm_types::{App, SpawnApp, TemplateAddr, WasmValue};

use crate::api::raw::encode_spawn_app;

/// Builds a raw representation for `spawn-app`
/// Should be used for testing only.
pub struct SpawnAppBuilder {
    version: Option<u32>,
    template: Option<TemplateAddr>,
    name: Option<String>,
    ctor_idx: Option<u16>,
    calldata: Option<Vec<u8>>,
}

///
/// # Example
///
/// ```rust
/// use svm_types::{App, SpawnApp, WasmValue, Address};
/// use svm_nibble::NibbleIter;
/// use svm_codec::api::{raw::decode_spawn_app, builder::SpawnAppBuilder};
///
/// let template = Address::of("@template").into();
/// let name = "My App".to_string();
/// let ctor_idx = 2;
/// let calldata = vec![0x10, 0x20, 0x30];
/// let ctor_args = vec![WasmValue::I32(0x40), WasmValue::I64(0x50)];
///
/// let bytes = SpawnAppBuilder::new()
///             .with_version(0)
///             .with_template(&template)
///             .with_name(&name)
///             .with_ctor_index(ctor_idx)
///             .with_ctor_buf(&calldata)
///             .with_ctor_args(&ctor_args)
///             .build();
///
/// let mut iter = NibbleIter::new(&bytes);
/// let actual = decode_spawn_app(&mut iter).unwrap();
/// let expected = SpawnApp {
///                  app: App { version: 0, name, template },
///                  ctor_idx,
///                  calldata,
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
            name: None,
            ctor_idx: None,
            calldata: None,
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_template(mut self, template: &TemplateAddr) -> Self {
        self.template = Some(template.clone());
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_ctor_index(mut self, ctor_idx: u16) -> Self {
        self.ctor_idx = Some(ctor_idx);
        self
    }

    pub fn with_calldata(mut self, calldata: &Vec<u8>) -> Self {
        self.calldata = Some(calldata.clone());
        self
    }

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let template = self.template.unwrap();
        let name = self.name.unwrap();
        let ctor_idx = self.ctor_idx.unwrap();

        let calldata = match self.calldata {
            None => vec![],
            Some(calldata) => calldata.to_vec(),
        };

        let spawn = SpawnApp {
            app: App {
                version,
                name,
                template,
            },
            ctor_idx,
            calldata,
        };

        let mut w = NibbleWriter::new();

        encode_spawn_app(&spawn, &mut w);

        w.into_bytes()
    }
}
