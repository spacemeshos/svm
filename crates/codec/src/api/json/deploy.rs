use serde::{Deserialize, Serialize};

use svm_layout::{FixedLayoutBuilder, Id, Layout};
use svm_types::{CodeSection, CtorsSection, DataSection, HeaderSection};

use super::{serde_types::HexBlob, JsonSerdeUtils};
use crate::api::builder::TemplateBuilder;
use crate::api::json::JsonError;
use crate::template;

///
/// ```json
/// {
///   "name": "...",          // string
///   "svm_version": "...",   // number (`u32`)
///   "code_version": "...",  // number (`u32`)
///   "desc": "...",          // string
///   "code": "...",          // string (represents a `blob`)
///   "data": "",             // string (represents a `blob`)
///   "ctors": ["", ""],      // string[]
/// }
/// ```
pub fn deploy_template(json: &str) -> Result<Vec<u8>, JsonError> {
    let deploy = DecodedDeploy::from_json_str(json)?;
    let layout = to_data_layout(deploy.data.0)?;
    let code = CodeSection::new_fixed(deploy.code.0, deploy.svm_version);
    let data = DataSection::with_layout(layout);
    let ctors = CtorsSection::new(deploy.ctors);
    let header = HeaderSection::new(deploy.code_version, deploy.name, deploy.desc);

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
            path: "data".to_string(),
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DecodedDeploy {
    svm_version: u32,
    code_version: u32,
    name: String,
    desc: String,
    code: HexBlob<Vec<u8>>,
    data: HexBlob<Vec<u8>>,
    ctors: Vec<String>,
}

impl JsonSerdeUtils for DecodedDeploy {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    use serde_json::json;
    use svm_layout::FixedLayout;

    #[test]
    fn json_deploy_template_missing_svm_version() {
        let json = json!({}).to_string();

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "svm_version".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_code_version() {
        let json = json!({
            "svm_version": 1
        })
        .to_string();

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "code_version".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_name() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2
        })
        .to_string();

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "name".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_desc() {
        let json = json!({
            "svm_version": 1,
            "code_version": 2,
            "name": "My Template",
        })
        .to_string();

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "desc".to_string(),
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
        })
        .to_string();

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "code".to_string(),
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
        })
        .to_string();

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "data".to_string(),
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
        })
        .to_string();

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::MissingField {
                field_name: "ctors".to_string(),
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
        })
        .to_string();

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
