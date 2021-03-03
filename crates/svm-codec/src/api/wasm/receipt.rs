use serde_json::Value;

use super::wasm_buf_apply;
use crate::api::{self, json::JsonError};

/// Decodes a binary Receipt given as an offset to a Wasm buffer,
/// and then returs an offset to a new Wasm buffer holding the decoded Receipt
/// in a JSON format.
pub fn decode_receipt(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: &Value| {
        let json = api::json::decode_receipt(json)?;

        api::json::to_bytes(&json)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use svm_types::gas::MaybeGas;
    use svm_types::receipt::SpawnAppReceipt;
    use svm_types::{Address, State};

    use crate::api::json;
    use crate::api::wasm::{free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER};

    use serde_json::{json, Value};

    #[test]
    fn wasm_decode_receipt_valid() {
        let app: Address = [0x10; 20].into();
        let state: State = [0xA0; 32].into();
        let logs = Vec::new();

        let receipt = SpawnAppReceipt {
            version: 0,
            success: true,
            error: None,
            app_addr: Some(app.into()),
            init_state: Some(state),
            returndata: Some(vec![0x10, 0x20]),
            gas_used: MaybeGas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_app_receipt(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = json!({ "data": data });
        let json = serde_json::to_string(&json).unwrap();

        let json_buf = to_wasm_buffer(json.as_bytes());
        let receipt_buf = decode_receipt(json_buf).unwrap();

        let data = wasm_buffer_data(receipt_buf);
        assert_eq!(data[0], BUF_OK_MARKER);

        let json: Value = serde_json::from_slice(&data[1..]).unwrap();

        assert_eq!(
            json,
            json!({
                "success": true,
                "type": "spawn-app",
                "app": "1010101010101010101010101010101010101010",
                "gas_used": 10,
                "returndata": "1020",
                "state": "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0",
                "logs": []
            })
        );

        free(json_buf);
        free(receipt_buf);
    }
}
