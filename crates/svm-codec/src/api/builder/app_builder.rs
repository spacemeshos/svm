use svm_nibble::NibbleWriter;
use svm_types::{App, SpawnApp, TemplateAddr, WasmValue};

use crate::api::raw::encode_spawn_app;

/// Builds a raw representation for `spawn-app`
/// Should be used for testing only.
pub struct SpawnAppBuilder {
    version: Option<u32>,
    template: Option<TemplateAddr>,
    name: Option<String>,
    ctor: Option<String>,
    calldata: Option<Vec<u8>>,
}

///
/// # Example
///
/// ```rust
/// use svm_types::{App, SpawnApp, Address};
/// use svm_nibble::NibbleIter;
/// use svm_codec::api::{raw::decode_spawn_app, builder::SpawnAppBuilder};
///
/// let template = Address::of("@template").into();
/// let name = "My App".to_string();
/// let ctor = "initialize";
/// let calldata = vec![0x10, 0x20, 0x30];
///
/// let bytes = SpawnAppBuilder::new()
///             .with_version(0)
///             .with_template(&template)
///             .with_name(&name)
///             .with_ctor(ctor)
///             .with_calldata(&calldata)
///             .build();
///
/// let mut iter = NibbleIter::new(&bytes);
/// let actual = decode_spawn_app(&mut iter).unwrap();
/// let expected = SpawnApp {
///                  app: App { version: 0, name, template },
///                  ctor,
///                  calldata,
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
            ctor: None,
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

    pub fn with_ctor(mut self, ctor: &str) -> Self {
        self.ctor = Some(ctor.to_string());
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
        let ctor = self.ctor.unwrap();

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
            ctor,
            calldata,
        };

        let mut w = NibbleWriter::new();

        encode_spawn_app(&spawn, &mut w);

        w.into_bytes()
    }
}
