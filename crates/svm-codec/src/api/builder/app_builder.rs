use svm_types::{App, SpawnApp, TemplateAddr, WasmValue};

use crate::{api::raw::encode_spawn_app, nibble::NibbleWriter};

/// Builds a raw representation for `spawn-app`
/// Should be used for testing only.
pub struct SpawnAppBuilder {
    version: Option<u32>,
    template: Option<TemplateAddr>,
    ctor_idx: Option<u16>,
    ctor_buf: Option<Vec<u8>>,
    ctor_args: Option<Vec<WasmValue>>,
}

///
/// # Example
///
/// ```rust
/// use svm_types::{App, SpawnApp, WasmValue};
/// use svm_common::Address;
/// use svm_codec:{testing::SpawnAppBuilder, decode_spawn_app, NibbleIter};
///
/// let template = Address::of("@template").into();
/// let ctor_idx = 2;
/// let ctor_buf = vec![0x10, 0x20, 0x30];
/// let ctor_args = vec![WasmValue::I32(0x40), WasmValue::I64(0x50)];
///
/// let bytes = SpawnAppBuilder::new()
///             .with_version(0)
///             .with_template(&template)
///             .with_ctor_index(ctor_idx)
///             .with_ctor_buf(&ctor_buf)
///             .with_ctor_args(&ctor_args)
///             .build();
///
/// let mut iter = NibbleIter::new(&bytes[..]);
/// let actual = decode_spawn_app(&mut iter).unwrap();
/// let expected = SpawnApp {
///                  app: App { version: 0, template },
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

    pub fn with_template(mut self, template: &TemplateAddr) -> Self {
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

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let template = self.template.unwrap();
        let ctor_idx = self.ctor_idx.unwrap();

        let ctor_buf = match self.ctor_buf {
            None => vec![],
            Some(buf) => buf.to_vec(),
        };

        let ctor_args = match self.ctor_args {
            None => vec![],
            Some(args) => args.to_vec(),
        };

        let spawn = SpawnApp {
            app: App { version, template },
            ctor_idx,
            ctor_buf,
            ctor_args,
        };

        let mut w = NibbleWriter::new();

        encode_spawn_app(&spawn, &mut w);

        w.into_bytes()
    }
}
