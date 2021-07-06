use serde_json::Value;

use crate::api::builder::TemplateBuilder;
use crate::api::json::{self, JsonError};
use crate::template;

use svm_layout::{Id, Layout, LayoutBuilder};
use svm_types::{CodeSection, CtorsSection, DataSection, HeaderSection};

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
pub fn deploy_template(json: &Value) -> Result<Vec<u8>, JsonError> {
    let svm_version = json::as_u32(json, "svm_version")?;
    let code_version = json::as_u32(json, "code_version")?;
    let name = json::as_string(json, "name")?;
    let desc = json::as_string(json, "desc")?;
    let wasm = json::as_blob(json, "code")?;
    let layout = json::as_blob(json, "data")?;
    let layout = to_data_layout(layout)?;

    let mut ctors = Vec::new();

    for ctor in json::as_array(json, "ctors")? {
        let ctor = ctor.as_str().unwrap();

        ctors.push(ctor.to_string());
    }

    let code = CodeSection::new_fixed(wasm, svm_version);
    let data = DataSection::with_layout(layout);
    let ctors = CtorsSection::new(ctors);
    let header = HeaderSection::new(code_version, name, desc);

    let template = TemplateBuilder::default()
        .with_code(code)
        .with_data(data)
        .with_ctors(ctors)
        .with_header(header)
        .build();

    let bytes = template::encode(&template);

    Ok(bytes)
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
    let mut builder = LayoutBuilder::with_capacity(data.len());

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
                reason: "value `null` isn\'t a number".to_string(),
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
                reason: "value `null` isn\'t a number".to_string(),
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
                reason: "value `null` isn\'t a string".to_string(),
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
                reason: "value `null` isn\'t a string".to_string(),
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
                reason: "value `null` isn\'t a string".to_string(),
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
                reason: "value `null` isn\'t a string".to_string(),
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
                reason: "value `null` isn\'t an Array".to_string(),
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
