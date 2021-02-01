use svm_layout::DataLayout;
use svm_types::AppTemplate;

use crate::template;

/// Builds a raw representation for `deploy-template`
/// Should be used for testing only.
pub struct DeployAppTemplateBuilder {
    version: Option<u16>,
    name: Option<String>,
    code: Option<Vec<u8>>,
    data: Option<DataLayout>,
}

///
/// # Example
///  
/// ```rust
/// use std::io::Cursor;
///
/// use svm_types::AppTemplate;
/// use svm_codec::api::builder::DeployAppTemplateBuilder;
/// use svm_codec::template;
///
/// let layout = vec![5, 10].into();
///
/// let bytes = DeployAppTemplateBuilder::new()
///            .with_version(0)
///            .with_name("My Template")
///            .with_code(&[0xC, 0x0, 0xD, 0xE])
///            .with_data(&layout)
///            .build();
///
/// let mut cursor = Cursor::new(&bytes[..]);
/// let actual = template::decode_deploy_template(&mut cursor).unwrap();
///
/// let expected = AppTemplate {
///                  version: 0,
///                  name: "My Template".to_string(),
///                  code: vec![0xC, 0x0, 0xD, 0xE],
///                  data: layout
///                };
///
/// assert_eq!(expected, actual);
/// ```
///
#[allow(missing_docs)]
impl DeployAppTemplateBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            name: None,
            code: None,
            data: None,
        }
    }

    pub fn with_version(mut self, version: u16) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_code(mut self, code: &[u8]) -> Self {
        self.code = Some(code.to_vec());
        self
    }

    pub fn with_data(mut self, data: &DataLayout) -> Self {
        self.data = Some(data.clone());
        self
    }

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let name = self.name.unwrap();
        let code = self.code.unwrap();
        let data = self.data.unwrap();

        let app = AppTemplate {
            version,
            name,
            code,
            data,
        };

        let mut w = Vec::new();

        template::encode_deploy_template(&app, &mut w);

        w
    }
}
