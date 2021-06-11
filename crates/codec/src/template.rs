//!  `Template` Raw Format
//!
//!  +________________+
//!  |                |                                
//!  |    version     |               
//!  |________________|
//!  |                |
//!  | Header Section |
//!  |________________|
//!  |                |
//!  |  Code Section  |
//!  |________________|
//!  |                |
//!  | Data Section   |
//!  |________________|
//!  |                |
//!  | Ctors Section  |
//!  |________________|
//!  |                |
//!  | Schema Section | (Optional)
//!  |________________|
//!  |                |
//!  |  API Section   | (Optional)
//!  |________________|
//!  |                |
//!  | Deploy Section | (Optional, will be derived from the `Transaction Envelope`)
//!  +________________+
//!
//!

use std::io::Cursor;

use svm_types::Template;

use crate::ParseError;

pub fn encode(template: &Template, w: &mut Vec<u8>) {
    todo!()
}

pub fn decode(cursor: &mut Cursor<&[u8]>) -> Result<Template, ParseError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_deploy_template() {
        let template = Template {
            version: 0,
            name: "My Template".to_string(),
            code: vec![0x0C, 0x00, 0x0D, 0x0E],
            data: vec![5, 10].into(),
            ctors: vec!["init".into(), "start".into()],
        };

        let mut bytes = Vec::new();
        encode(&template, &mut bytes);

        let mut cursor = Cursor::new(&bytes);

        let decoded = decode(&mut cursor).unwrap();

        assert_eq!(template, decoded);
    }
}
