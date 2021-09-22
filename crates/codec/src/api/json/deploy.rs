use std::convert::TryInto;

use svm_layout::{FixedLayout, Layout};
use svm_types::{CodeSection, CtorsSection, DataSection, HeaderSection, Template};

use super::{get_field, parse_json, serde_types::HexBlob, JsonError};
use crate::template;

///
/// ```json
/// {
///   "svm_version": "...",   // number (`u32`)
///   "code_version": "...",  // number (`u32`)
///   "name": "...",          // string
///   "desc": "...",          // string
///   "code": "...",          // string (represents a `blob`)
///   "data": "",             // string (represents a `blob`)
///   "ctors": ["", ""],      // string[]
/// }
/// ```
pub fn deploy_template(json: &str) -> Result<Vec<u8>, JsonError> {
    let json = &mut parse_json(json)?;

    let svm_version = get_field::<u32>(json, "svm_version")?;
    let code_version = get_field(json, "code_version")?;
    let name = get_field(json, "name")?;
    let desc = get_field(json, "desc")?;
    let code = get_field::<HexBlob<Vec<u8>>>(json, "code")?.0;
    let data = get_field::<HexBlob<Vec<u8>>>(json, "data")?.0;
    let ctors = get_field::<Vec<String>>(json, "ctors")?;

    let layout = to_data_layout(data)?;
    let code = CodeSection::new_fixed(code, svm_version);
    let data = DataSection::with_layout(layout);
    let ctors = CtorsSection::new(ctors);
    let header = HeaderSection::new(code_version, name, desc);

    let template = Template::new(code, data, ctors).with_header(Some(header));

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
        .map(|bytes| u32::from_be_bytes(bytes.try_into().unwrap()))
        .collect();

    let fixed = FixedLayout::from_byte_sizes(0, &data);
    let layout = Layout::Fixed(fixed);

    Ok(layout)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use std::io::Cursor;

    use svm_layout::FixedLayout;

    use super::*;

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
        let fixed = FixedLayout::from_byte_sizes(0, &[1, 3]);
        let data = DataSection::with_layout(Layout::Fixed(fixed));
        let ctors = CtorsSection::new(vec!["init".into(), "start".into()]);
        let header = HeaderSection::new(2, "My Template".into(), "A few words".into());

        let expected = Template::new(code, data, ctors).with_header(Some(header));

        assert_eq!(actual, expected);
    }
}
