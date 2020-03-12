use crate::{
    raw::{encode_deploy_template, helpers, NibbleWriter},
    types::AppTemplate,
};

use svm_common::Address;

/// Builds a raw representation for `deploy-template`
/// Should be used for testing only.
pub struct DeployAppTemplateBuilder {
    version: Option<u32>,
    name: Option<String>,
    page_count: Option<u16>,
    code: Option<Vec<u8>>,
}

///
/// # Example
///  
/// ```rust
/// use svm_app::{types::AppTemplate, testing::AppTemplateBuilder, raw::parse_template};
///
/// let bytes = DeployAppTemplateBuilder::new()
///            .with_version(0)
///            .with_name("My Template")
///            .with_page_count(10)
///            .with_code(&[0xC, 0x0, 0xD, 0xE])
///            .build();
///
/// let actual = parse_deploy_template(&bytes[..]).unwrap();
///
/// let expected = AppTemplate {
///                  version: 0,
///                  name: "My Template".to_string(),
///                  page_count: 10,
///                  code: vec![0xC, 0x0, 0xD, 0xE]
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
            page_count: None,
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_page_count(mut self, page_count: u16) -> Self {
        self.page_count = Some(page_count);
        self
    }

    pub fn with_code(mut self, code: &[u8]) -> Self {
        self.code = Some(code.to_vec());
        self
    }

    pub fn build(mut self) -> Vec<u8> {
        let version = self.version.unwrap();
        let name = self.name.unwrap();
        let page_count = self.page_count.unwrap();
        let code = self.code.unwrap();

        let app = AppTemplate {
            version,
            name,
            page_count,
            code,
        };

        let mut w = NibbleWriter::new();

        encode_deploy_template(&app, &mut w);

        w.into_bytes()
    }
}
