use serde_json::Value;

use crate::{
    api::json::{self, JsonError},
    nibble::NibbleWriter,
};

///
/// ```json
/// {
///   abi: {},
///   data: {}
/// }
/// ```
pub fn func_buf(json: &Value) -> Result<Vec<u8>, JsonError> {
    todo!()
}
