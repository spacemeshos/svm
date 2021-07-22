use serde_json::Value;

use super::wasm_buf_apply;
use crate::api::{self, json::JsonError};

/// Decodes a binary Receipt given as an offset to a Wasm buffer,
/// and then returns an offset to a new Wasm buffer holding the decoded Receipt
/// in a JSON format.
pub fn decode_receipt(offset: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(offset, |json: Value| {
        let json = api::json::decode_receipt(json)?;

        api::json::to_bytes(&json)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use svm_types::{Address, Gas, SpawnReceipt, State};

    use crate::api::json;
    use crate::api::wasm::{free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER};

    use serde_json::{json, Value};

    #[test]
    fn wasm_decode_receipt_valid() {
        let account = Address::repeat(0x10);
        let state = State::repeat(0xA0);
        let logs = Vec::new();

        let receipt = SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(account.into()),
            init_state: Some(state),
            returndata: Some(vec![0x10, 0x20]),
            gas_used: Gas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_spawn(&receipt);
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
                "type": "spawn-account",
                "account": "1010101010101010101010101010101010101010",
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
