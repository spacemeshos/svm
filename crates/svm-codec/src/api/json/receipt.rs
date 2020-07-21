use serde_json::{json, Value};

use crate::api::json::{self, JsonError};
use crate::api::raw;

use svm_types::receipt::{ExecReceipt, ReceiptError, SpawnAppReceipt, TemplateReceipt};

pub fn decode_receipt(json: &Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    assert!(bytes.len() > 0);

    let receipt = raw::decode_receipt(&bytes);

    todo!()
    // let json = match receipt {
    //     Receipt::DeployTemplate(r) => todo!(),
    //     Receipt::SpawnApp(r) => todo!(),
    //     Receipt::ExecApp(r) => todo!(),
    // };

    // Ok(json)
}

// fn decode_spawn_app(receipt: &ClientAppReceipt) -> Value {
//     match receipt {
//         ClientAppReceipt::Success { addr, init_state, ctor_returns, gas_used, logs }

//         json!({
//             "success": true,
//             "app": json::addr_to_str(app.inner()),
//             "state": json::state_to_str(state),
//             "returns": json::wasm_values_to_json(returns),
//             "gas_used": json::gas_to_json(&gas_used),
//             "logs": json::logs_to_json(&receipt.logs),
//         },
//         _ => todo!()
//     }
// }

// fn error_receipt(receipt: &SpawnAppReceipt) -> Value {
//     let gas_used = receipt.get_gas_used();
//     let error = receipt.get_error();

//     json!({
//         "success": false,
//         "error": error.to_string(),
//         "gas_used": json::gas_to_json(&gas_used),
//         "logs": json::logs_to_json(&receipt.logs),
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    use super::json;

    use svm_types::{gas::MaybeGas, receipt::Log, Address, AppAddr, State, WasmValue};

    #[test]
    #[ignore]
    fn spawn_app_receipt_success() {
        let app: Address = [0x10; 20][..].into();
        let state: State = [0xA0; 32][..].into();

        let logs = vec![
            Log {
                msg: b"Log entry #1".to_vec(),
                code: 100,
            },
            Log {
                msg: b"Log entry #2".to_vec(),
                code: 200,
            },
        ];

        let receipt = SpawnAppReceipt {
            success: true,
            error: None,
            app_addr: Some(app.into()),
            init_state: Some(state),
            returns: Some(vec![WasmValue::I32(10), WasmValue::I64(20)]),
            gas_used: MaybeGas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_app_receipt(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "success": true,
                "app": "1010101010101010101010101010101010101010",
                "gas_used": 10,
                "returns": ["10i32", "20i64"],
                "state": "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0",
                "logs": [
                    {"msg": "Log entry #1", "code": 100,},
                    {"msg": "Log entry #2", "code": 200}
                ]
            })
        );
    }

    #[test]
    #[ignore]
    fn spawn_app_receipt_error() {
        let logs = vec![Log {
            msg: b"Reached OOG".to_vec(),
            code: 0,
        }];

        let receipt = SpawnAppReceipt {
            success: false,
            error: Some(ReceiptError::OOG),
            app_addr: None,
            init_state: None,
            returns: None,
            gas_used: MaybeGas::with(1000),
            logs,
        };

        let bytes = crate::receipt::encode_app_receipt(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
               "success": false,
               "error": "OOG",
               "gas_used": 1000,
               "logs": [{"code": 0, "msg": "Reached OOG"}],
            })
        );
    }
}
