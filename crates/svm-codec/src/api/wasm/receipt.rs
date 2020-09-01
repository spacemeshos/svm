use serde_json::Value;

use super::wasm_buf_apply;
use crate::{
    api,
    api::json::{self, JsonError},
};

pub fn decode_receipt(ptr: usize) -> Result<usize, JsonError> {
    wasm_buf_apply(ptr, |json: &Value| {
        let json = api::json::decode_receipt(json)?;

        api::json::to_bytes(&json)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    use svm_nibble::NibbleIter;
    use svm_types::{gas::MaybeGas, receipt::SpawnAppReceipt, Address, State, WasmValue};

    use crate::api::wasm::{
        error_as_string, free, to_wasm_buffer, wasm_buffer_data, BUF_OK_MARKER,
    };

    use serde_json::{json, Value};

    #[test]
    fn wasm_decode_receipt_valid() {
        let app: Address = [0x10; 20].into();
        let state: State = [0xA0; 32].into();
        let logs = Vec::new();

        let receipt = SpawnAppReceipt {
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
