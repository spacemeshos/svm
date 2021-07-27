use svm_types::{Account, SpawnAccount, TemplateAddr};

use crate::spawn;

/// Builds a binary representation for [`SpawnAccount`]
///
/// Should be used mainly for testing only.
pub struct SpawnBuilder {
    version: Option<u16>,
    template: Option<TemplateAddr>,
    name: Option<String>,
    ctor_name: Option<String>,
    calldata: Option<Vec<u8>>,
}

///
/// # Examples
///
/// ```rust
/// use std::io::Cursor;
///
/// use svm_types::{Account, SpawnAccount, TemplateAddr};
/// use svm_codec::api::builder::SpawnBuilder;
/// use svm_codec::spawn;
///
/// let template_addr = TemplateAddr::of("@template");
/// let name = "My Account".to_string();
/// let ctor_name = "initialize";
/// let calldata = vec![0x10, 0x20, 0x30];
///
/// let bytes = SpawnBuilder::new()
///             .with_version(0)
///             .with_template(&template_addr)
///             .with_name(&name)
///             .with_ctor(ctor_name)
///             .with_calldata(&calldata)
///             .build();
///
/// let mut cursor = Cursor::new(&bytes[..]);
/// let actual = spawn::decode(&mut cursor).unwrap();
/// let expected = SpawnAccount {
///                  version: 0,
///                  account: Account { name, template_addr },
///                  ctor_name: ctor_name.to_string(),
///                  calldata,
///                };
///
//// assert_eq!(expected, actual);
/// ```
///
#[allow(missing_docs)]
impl SpawnBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            template: None,
            name: None,
            ctor_name: None,
            calldata: None,
        }
    }

    pub fn with_version(mut self, version: u16) -> Self {
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

    pub fn with_ctor(mut self, ctor_name: &str) -> Self {
        self.ctor_name = Some(ctor_name.to_string());
        self
    }

    pub fn with_calldata(mut self, calldata: &[u8]) -> Self {
        self.calldata = Some(calldata.to_vec());
        self
    }

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let template_addr = self.template.unwrap();
        let name = self.name.unwrap();
        let ctor_name = self.ctor_name.unwrap();

        let calldata = match self.calldata {
            None => vec![],
            Some(calldata) => calldata.to_vec(),
        };

        let spawn = SpawnAccount {
            version,
            account: Account::new(template_addr, name),
            ctor_name,
            calldata,
        };

        let mut w = Vec::new();

        spawn::encode(&spawn, &mut w);

        w
    }
}
