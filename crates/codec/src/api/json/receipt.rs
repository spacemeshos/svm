use serde_json::{json, Value};

use crate::api::json::{self, JsonError};
use crate::receipt;

use svm_types::RuntimeError;
use svm_types::{CallReceipt, DeployReceipt, Receipt, ReceiptLog, SpawnReceipt};

/// Given a binary Receipt wrapped inside a JSON,
/// decodes it into a user-friendly JSON.
pub fn decode_receipt(json: Value) -> Result<Value, JsonError> {
    let data = json::as_string(&json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    assert!(bytes.len() > 0);

    let receipt = receipt::decode_receipt(&bytes);
    let ty = receipt_type(&receipt);

    let json = if receipt.success() {
        match receipt {
            Receipt::Deploy(receipt) => decode_deploy(&receipt, ty),
            Receipt::Spawn(receipt) => decode_spawn(&receipt, ty),
            Receipt::Call(receipt) => decode_call(&receipt, ty),
        }
    } else {
        let ty = receipt_type(&receipt);
        let logs = receipt.logs();
        let err = receipt.error();

        decode_error(ty, err, logs)
    };

    Ok(json)
}

fn receipt_type(receipt: &Receipt) -> &'static str {
    match receipt {
        Receipt::Deploy(..) => "deploy-template",
        Receipt::Spawn(..) => "spawn-account",
        Receipt::Call(..) => "call-account",
    }
}

fn decode_error(ty: &'static str, err: &RuntimeError, logs: &[ReceiptLog]) -> Value {
    let mut json = {
        match err {
            RuntimeError::OOG => json!({
                "err_type": "oog",
            }),
            RuntimeError::TemplateNotFound(template_addr) => json!({
                "err_type": "template-not-found",
                "template_addr": json::addr_to_str(template_addr.inner()),
            }),
            RuntimeError::AccountNotFound(account_addr) => json!({
                "err_type": "account-not-found",
                "account_addr": json::addr_to_str(account_addr.inner()),
            }),
            RuntimeError::CompilationFailed {
                account_addr,
                template_addr,
                msg,
            } => json!({
                "err_type": "compilation-failed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "account_addr": json::addr_to_str(account_addr.inner()),
                "message": msg,
            }),
            RuntimeError::InstantiationFailed {
                account_addr,
                template_addr,
                msg,
            } => json!({
                "err_type": "instantiation-failed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "account_addr": json::addr_to_str(account_addr.inner()),
                "message": msg,
            }),
            RuntimeError::FuncNotFound {
                account_addr,
                template_addr,
                func,
            } => json!({
                "err_type": "function-not-found",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "account_addr": json::addr_to_str(account_addr.inner()),
                "func": func,
            }),
            RuntimeError::FuncFailed {
                account_addr,
                template_addr,
                func,
                msg,
            } => json!({
                "err_type": "function-failed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "account_addr": json::addr_to_str(account_addr.inner()),
                "func": func,
                "message": msg,
            }),
            RuntimeError::FuncNotAllowed {
                account_addr,
                template_addr,
                func,
                msg,
            } => json!({
                "err_type": "function-not-allowed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "account_addr": json::addr_to_str(account_addr.inner()),
                "func": func,
                "message": msg,
            }),
            RuntimeError::FuncInvalidSignature {
                account_addr,
                template_addr,
                func,
            } => json!({
                "err_type": "function-invalid-signature",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "account_addr": json::addr_to_str(account_addr.inner()),
                "func": func,
            }),
        }
    };

    let logs = json::logs_to_json(logs);

    let map: &mut serde_json::Map<String, Value> = json.as_object_mut().unwrap();
    let mut map: serde_json::Map<String, Value> = std::mem::take(map);

    map.insert("type".into(), Value::String(ty.into()));
    map.insert("success".into(), Value::Bool(false));
    map.insert("logs".into(), Value::Array(logs));

    map.into()
}

fn decode_deploy(receipt: &DeployReceipt, ty: &'static str) -> Value {
    debug_assert!(receipt.success);
    debug_assert!(receipt.error.is_none());

    let DeployReceipt {
        addr,
        gas_used,
        logs,
        ..
    } = receipt;

    json!({
        "type": ty,
        "success": true,
        "addr": json::addr_to_str(addr.as_ref().unwrap().inner()),
        "gas_used": json::gas_to_json(&gas_used),
        "logs": json::logs_to_json(&logs),
    })
}

fn decode_spawn(receipt: &SpawnReceipt, ty: &'static str) -> Value {
    debug_assert!(receipt.success);
    debug_assert!(receipt.error.is_none());

    let SpawnReceipt {
        account_addr,
        init_state,
        returndata,
        gas_used,
        logs,
        ..
    } = receipt;

    json!({
        "type": ty,
        "success": true,
        "account": json::addr_to_str(account_addr.as_ref().unwrap().inner()),
        "state": json::state_to_str(init_state.as_ref().unwrap()),
        "returndata": json::bytes_to_str(returndata.as_ref().unwrap()),
        "gas_used": json::gas_to_json(&gas_used),
        "logs": json::logs_to_json(&logs),
    })
}

fn decode_call(receipt: &CallReceipt, ty: &'static str) -> Value {
    debug_assert!(receipt.success);
    debug_assert!(receipt.error.is_none());

    let CallReceipt {
        new_state,
        returndata,
        gas_used,
        logs,
        ..
    } = receipt;

    json!({
        "type": ty,
        "success": true,
        "new_state": json::state_to_str(new_state.as_ref().unwrap()),
        "returndata": json::bytes_to_str(returndata.as_ref().unwrap()),
        "gas_used": json::gas_to_json(&gas_used),
        "logs": json::logs_to_json(&logs),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::json;

    use svm_types::{Address, Gas, ReceiptLog, State};

    #[test]
    fn decode_receipt_deploy_success() {
        let template = Address::repeat(0x10);

        let logs = vec![
            ReceiptLog {
                msg: b"Log entry #1".to_vec(),
                code: 100,
            },
            ReceiptLog {
                msg: b"Log entry #2".to_vec(),
                code: 200,
            },
        ];

        let receipt = DeployReceipt {
            version: 0,
            success: true,
            error: None,
            addr: Some(template.into()),
            gas_used: Gas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_deploy(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "success": true,
                "type": "deploy-template",
                "addr": "1010101010101010101010101010101010101010",
                "gas_used": 10,
                "logs": [
                    {"msg": "Log entry #1", "code": 100},
                    {"msg": "Log entry #2", "code": 200}
                ]
            })
        );
    }

    #[test]
    fn decode_receipt_spawn_success() {
        let account = Address::repeat(0x10);
        let state = State::repeat(0xA0);

        let logs = vec![
            ReceiptLog {
                msg: b"Log entry #1".to_vec(),
                code: 100,
            },
            ReceiptLog {
                msg: b"Log entry #2".to_vec(),
                code: 200,
            },
        ];

        let receipt = SpawnReceipt {
            version: 0,
            success: true,
            error: None,
            account_addr: Some(account.into()),
            init_state: Some(state),
            returndata: Some(vec![0x10, 0x20, 0x30]),
            gas_used: Gas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_spawn(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "success": true,
                "type": "spawn-account",
                "account": "1010101010101010101010101010101010101010",
                "gas_used": 10,
                "returndata": "102030",
                "state": "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0",
                "logs": [
                    {"msg": "Log entry #1", "code": 100},
                    {"msg": "Log entry #2", "code": 200}
                ]
            })
        );
    }

    #[test]
    fn decode_receipt_spawn_error() {
        let logs = vec![ReceiptLog {
            msg: b"Reached OOG".to_vec(),
            code: 0,
        }];

        let receipt = SpawnReceipt {
            version: 0,
            success: false,
            error: Some(RuntimeError::OOG),
            account_addr: None,
            init_state: None,
            returndata: None,
            gas_used: Gas::with(1000),
            logs,
        };

        let bytes = crate::receipt::encode_spawn(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
               "type": "spawn-account",
               "success": false,
               "err_type": "oog",
               "logs": [{"code": 0, "msg": "Reached OOG"}],
            })
        );
    }

    #[test]
    fn decode_receipt_call_success() {
        let state = State::repeat(0xA0);

        let logs = vec![
            ReceiptLog {
                msg: b"Log entry #1".to_vec(),
                code: 100,
            },
            ReceiptLog {
                msg: b"Log entry #2".to_vec(),
                code: 200,
            },
        ];

        let receipt = CallReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(state),
            returndata: Some(vec![0x10, 0x20]),
            gas_used: Gas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_call(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "success": true,
                "type": "call-account",
                "gas_used": 10,
                "returndata": "1020",
                "new_state": "A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0A0",
                "logs": [
                    {"msg": "Log entry #1", "code": 100},
                    {"msg": "Log entry #2", "code": 200}
                ]
            })
        );
    }
}
