use byteorder::{BigEndian, ByteOrder};
use serde_json::Value;

use crate::{
    api::json::{self, JsonError},
    template, NibbleWriter,
};

use svm_common::Address;
use svm_layout::{DataLayout, DataLayoutBuilder};
use svm_types::{AppTemplate, WasmValue};

///
/// ```json
/// {
///   version: 0,   // number
///   name: '...',  // string
///   code: '...',  // string (represents a `blob`)
///   data: '',     // string (represents a `blob`)
/// }
/// ```
pub fn deploy_template(json: &Value) -> Result<Vec<u8>, JsonError> {
    let version = json::as_u32(json, "version")?;
    let name = json::as_string(json, "name")?;
    let code = json::as_blob(json, "code")?;
    let data = json::as_blob(json, "data")?;
    let data = to_data_layout(data)?;

    let template = AppTemplate {
        version,
        name,
        code,
        data,
    };

    let mut w = NibbleWriter::new();
    template::encode_deploy_template(&template, &mut w);

    let bytes = w.into_bytes();
    Ok(bytes)
}

fn to_data_layout(blob: Vec<u8>) -> Result<DataLayout, JsonError> {
    if blob.len() % 4 != 0 {
        return Err(JsonError::InvalidField {
            field: "data".to_string(),
            reason: "invalid value".to_string(),
        });
    }

    let data: Vec<u32> = blob
        .chunks_exact(4)
        .map(|buf| BigEndian::read_u32(&buf))
        .collect();

    let mut builder = DataLayoutBuilder::new();
    builder.extend_from_slice(&data);
    let data = builder.build();

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    use crate::NibbleIter;

    #[test]
    fn json_deploy_template_missing_version() {
        let json = json!({});

        let err = deploy_template(&json).unwrap_err();
        assert_eq!(
            err,
            JsonError::InvalidField {
                field: "version".to_string(),
                reason: "value `null` isn\'t a number".to_string(),
            }
        );
    }

    #[test]
    fn json_deploy_template_missing_name() {
        let json = json!({
            "version": 0
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
    fn json_deploy_template_missing_code() {
        let json = json!({
            "version": 0,
            "name": "My Template",
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
            "version": 0,
            "name": "My Template",
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
    fn json_deploy_template_valid() {
        let json = json!({
            "version": 0,
            "name": "My Template",
            "code": "C0DE",
            "data": "0000000100000003"
        });

        let bytes = deploy_template(&json).unwrap();

        let mut iter = NibbleIter::new(&bytes[..]);
        let actual = crate::decode_deploy_template(&mut iter).unwrap();

        let expected = AppTemplate {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0xC0, 0xDE],
            data: vec![1, 3].into(),
        };

        assert_eq!(actual, expected);
    }
}
