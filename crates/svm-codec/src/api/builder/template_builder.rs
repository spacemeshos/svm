use svm_layout::Layout;
use svm_types::Template;

use crate::template;

/// Builds a raw representation for `deploy-template`
/// Should be used for testing only.
pub struct DeployTemplateBuilder {
    version: Option<u16>,
    name: Option<String>,
    code: Option<Vec<u8>>,
    layout: Option<Layout>,
    ctors: Option<Vec<String>>,
}

///
/// # Example
///  
/// ```rust
/// use std::io::Cursor;
///
/// use svm_types::Template;
/// use svm_codec::api::builder::DeployTemplateBuilder;
/// use svm_codec::template;
///
/// let layout = vec![5, 10].into();
/// let ctors = vec!["init".to_string()];
///
/// let bytes = DeployTemplateBuilder::new()
///            .with_version(0)
///            .with_name("My Template")
///            .with_code(&[0xC, 0x0, 0xD, 0xE])
///            .with_layout(&layout)
///            .with_ctors(&ctors)
///            .build();
///
/// let mut cursor = Cursor::new(&bytes[..]);
/// let actual = template::decode_deploy_template(&mut cursor).unwrap();
///
/// let expected = Template {
///                  version: 0,
///                  name: "My Template".to_string(),
///                  code: vec![0xC, 0x0, 0xD, 0xE],
///                  layout,
///                  ctors: vec!["init".to_string()]
///                };
///
/// assert_eq!(expected, actual);
/// ```
///
#[allow(missing_docs)]
impl DeployTemplateBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            version: None,
            name: None,
            code: None,
            layout: None,
            ctors: None,
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

    pub fn with_layout(mut self, data: &Layout) -> Self {
        self.layout = Some(data.clone());
        self
    }

    pub fn with_ctors(mut self, ctors: &[String]) -> Self {
        self.ctors = Some(ctors.to_vec());
        self
    }

    pub fn build(self) -> Vec<u8> {
        let version = self.version.unwrap();
        let name = self.name.unwrap();
        let code = self.code.unwrap();
        let layout = self.layout.unwrap();
        let ctors = self.ctors.unwrap();

        let app = Template {
            version,
            name,
            code,
            layout,
            ctors,
        };

        let mut w = Vec::new();

        template::encode_deploy_template(&app, &mut w);

        w
    }
}
