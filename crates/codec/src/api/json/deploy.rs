use serde::{Deserialize, Serialize};
use serde_json::Value as Json;

use svm_layout::{FixedLayoutBuilder, Id, Layout};
use svm_types::{CodeSection, CtorsSection, DataSection, HeaderSection};

use super::{HexBlob, TypeInformation};
use crate::api::builder::TemplateBuilder;
use crate::api::json::JsonError;
use crate::template;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DeployWrapper {
    svm_version: u32,
    code_version: u32,
    name: String,
    desc: String,
    code: HexBlob,
    data: HexBlob,
    ctors: Vec<String>,
}

impl DeployWrapper {
    fn new(json: &Json) -> Result<Self, JsonError> {
        serde_json::from_value(json.clone()).map_err(|e| JsonError::from_serde::<DeployWrapper>(e))
    }
}

impl TypeInformation for DeployWrapper {
    fn type_of_field_as_str(field: &str) -> Option<&str> {
        Some(match field {
            "svm_version" | "code_version" => "number",
            "name" | "desc" => "string",
            "ctors" => "Array",
            _ => "string",
        })
    }
}

///
/// ```json
/// {
///   name: '...',          // string
///   svm_version: '...',   // number (`u32`)
///   code_version: '...',  // number (`u32`)
///   desc: '...',          // string
///   code: '...',          // string (represents a `blob`)
///   data: '',             // string (represents a `blob`)
///   ctors: ['', ''],      // string[]
/// }
/// ```
pub fn deploy_template(json: &Json) -> Result<Vec<u8>, JsonError> {
    let wrapper = DeployWrapper::new(json)?;
    let layout = to_data_layout(wrapper.data.0)?;
    let code = CodeSection::new_fixed(wrapper.code.0, wrapper.svm_version);
    let data = DataSection::with_layout(layout);
    let ctors = CtorsSection::new(wrapper.ctors);
    let header = HeaderSection::new(wrapper.code_version, wrapper.name, wrapper.desc);

    let template = TemplateBuilder::default()
        .with_code(code)
        .with_data(data)
        .with_ctors(ctors)
        .with_header(header)
        .build();

    Ok(template::encode(&template))
}

fn to_data_layout(blob: Vec<u8>) -> Result<Layout, JsonError> {
    if blob.len() % 4 != 0 {
        return Err(JsonError::InvalidField {
            field: "data".to_string(),
            reason: "invalid value".to_string(),
        });
    }

    let data: Vec<u32> = blob
        .chunks_exact(4)
        .map(|buf| {
            let bytes: [u8; 4] = [buf[0], buf[1], buf[2], buf[3]];

            u32::from_be_bytes(bytes)
        })
        .collect();

    // Note: `LayoutBuilder` assume that the `first var id` is zero
    let mut builder = FixedLayoutBuilder::with_capacity(data.len());

    builder.set_first(Id(0));
    builder.extend_from_slice(&data);

    let fixed = builder.build();
    let layout = Layout::Fixed(fixed);

    Ok(layout)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    use serde_json::json;
    use svm_layout::FixedLayout;

    #[test]
    fn json_deploy_template_missing_svm_version() {
        let json = json!({});

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "svm_version".to_string(),
                reason: "value `null` isn\'t a(n) number".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_code_version() {
        let json = json!({
            "svm_version": 1
        });

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "code_version".to_string(),
                reason: "value `null` isn\'t a(n) number".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_name() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2
        });

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "name".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_desc() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2,
            "name": "My Template",
        });

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "desc".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_code() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2,
            "name": "My Template",
            "desc": "A few words"
        });

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "code".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_data() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2,
            "name": "My Template",
            "desc": "A few words",
            "code": "C0DE"
        });

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "data".to_string(),
                reason: "value `null` isn\'t a(n) string".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_ctors() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2,
            "name": "My Template",
            "desc": "A few words",
            "code": "C0DE",
            "data": "0000000100000003",
        });

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "ctors".to_string(),
                reason: "value `null` isn\'t a(n) Array".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_valid() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2,
            "name": "My Template",
            "desc": "A few words",
            "code": "C0DE",
            "data": "0000000100000003",
            "ctors": ["init", "start"]
        });

        let bytes = deploy_template(&json).unwrap();
        let cursor = Cursor::new(&bytes[..]);
        let actual = template::decode(cursor, None).unwrap();

        let code = CodeSection::new_fixed(vec![0xC0, 0xDE], 1);
        let fixed = FixedLayout::from(vec![1, 3]);
        let data = DataSection::with_layout(Layout::Fixed(fixed));
        let ctors = CtorsSection::new(vec!["init".into(), "start".into()]);
        let header = HeaderSection::new(2, "My Template".into(), "A few words".into());

        let expected = TemplateBuilder::default()
            .with_code(code)
            .with_data(data)
            .with_ctors(ctors)
            .with_header(header)
            .build();

        assert_eq!(actual, expected);
    }
}
