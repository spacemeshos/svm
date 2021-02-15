use serde_json::{json, Value};

use crate::api::json::{self, JsonError};
use crate::receipt;

use svm_types::receipt::{
    ExecReceipt, Log, ReceiptError, Receipt, SpawnAppReceipt, TemplateReceipt,
};

/// Given a binary Receipt wrappend inside a JSON,
/// decodes it into a user-friendly JSON.
pub fn decode_receipt(json: &Value) -> Result<Value, JsonError> {
    let data = json::as_string(json, "data")?;
    let bytes = json::str_to_bytes(&data, "data")?;

    assert!(bytes.len() > 0);

    let receipt = receipt::decode_receipt(&bytes);
    let ty = receipt_type(&receipt);

    let json = if receipt.success() {
        match receipt {
            Receipt::DeployTemplate(receipt) => decode_deploy_template(&receipt, ty),
            Receipt::SpawnApp(receipt) => decode_spawn_app(&receipt, ty),
            Receipt::ExecApp(receipt) => decode_exe_app(&receipt, ty),
        }
    } else {
        let ty = receipt_type(&receipt);
        let logs = receipt.get_logs();
        let err = receipt.get_error();

        decode_error(ty, err, logs)
    };

    Ok(json)
}

fn receipt_type(receipt: &Receipt) -> &'static str {
    match receipt {
        Receipt::DeployTemplate(..) => "deploy-template",
        Receipt::SpawnApp(..) => "spawn-app",
        Receipt::ExecApp(..) => "exec-app",
    }
}

fn decode_error(ty: &'static str, err: &ReceiptError, logs: &[Log]) -> Value {
    let mut json = {
        match err {
            ReceiptError::OOG => json!({
                "err_type": "oog",
            }),
            ReceiptError::TemplateNotFound(template_addr) => json!({
                "err_type": "template-not-found",
                "template_addr": json::addr_to_str(template_addr.inner()),
            }),
            ReceiptError::AppNotFound(app_addr) => json!({
                "err_type": "app-not-found",
                "app_addr": json::addr_to_str(app_addr.inner()),
            }),
            ReceiptError::CompilationFailed {
                app_addr,
                template_addr,
                msg,
            } => json!({
                "err_type": "compilation-failed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "app_addr": json::addr_to_str(app_addr.inner()),
                "message": msg,
            }),
            ReceiptError::InstantiationFailed {
                app_addr,
                template_addr,
                msg,
            } => json!({
                "err_type": "instantiation-failed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "app_addr": json::addr_to_str(app_addr.inner()),
                "message": msg,
            }),
            ReceiptError::FuncNotFound {
                app_addr,
                template_addr,
                func,
            } => json!({
                "err_type": "function-not-found",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "app_addr": json::addr_to_str(app_addr.inner()),
                "func": func,
            }),
            ReceiptError::FuncFailed {
                app_addr,
                template_addr,
                func,
                msg,
            } => json!({
                "err_type": "function-failed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "app_addr": json::addr_to_str(app_addr.inner()),
                "func": func,
                "message": msg,
            }),
            ReceiptError::FuncNotAllowed {
                app_addr,
                template_addr,
                func,
                msg,
            } => json!({
                "err_type": "function-not-allowed",
                "template_addr": json::addr_to_str(template_addr.inner()),
                "app_addr": json::addr_to_str(app_addr.inner()),
                "func": func,
                "message": msg,
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

fn decode_deploy_template(receipt: &TemplateReceipt, ty: &'static str) -> Value {
    debug_assert!(receipt.success);
    debug_assert!(receipt.error.is_none());

    let TemplateReceipt {
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
        "logs": json::logs_to_json(&receipt.logs),
    })
}

fn decode_spawn_app(receipt: &SpawnAppReceipt, ty: &'static str) -> Value {
    debug_assert!(receipt.success);
    debug_assert!(receipt.error.is_none());

    let SpawnAppReceipt {
        app_addr,
        init_state,
        returndata,
        gas_used,
        logs,
        ..
    } = receipt;

    json!({
        "type": ty,
        "success": true,
        "app": json::addr_to_str(app_addr.as_ref().unwrap().inner()),
        "state": json::state_to_str(init_state.as_ref().unwrap()),
        "returndata": json::bytes_to_str(returndata.as_ref().unwrap()),
        "gas_used": json::gas_to_json(&gas_used),
        "logs": json::logs_to_json(&receipt.logs),
    })
}

fn decode_exe_app(receipt: &ExecReceipt, ty: &'static str) -> Value {
    debug_assert!(receipt.success);
    debug_assert!(receipt.error.is_none());

    let ExecReceipt {
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
        "logs": json::logs_to_json(&receipt.logs),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::json;

    use svm_types::{gas::MaybeGas, receipt::Log, Address, AppAddr, State, WasmValue};

    #[test]
    fn decode_receipt_deploy_template_receipt_success() {
        let template: Address = [0x10; 20].into();

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

        let receipt = TemplateReceipt {
            version: 0,
            success: true,
            error: None,
            addr: Some(template.into()),
            gas_used: MaybeGas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_template_receipt(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(&json!({ "data": data })).unwrap();

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
    fn decode_receipt_spawn_app_receipt_success() {
        let app: Address = [0x10; 20].into();
        let state: State = [0xA0; 32].into();

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
            version: 0,
            success: true,
            error: None,
            app_addr: Some(app.into()),
            init_state: Some(state),
            returndata: Some(vec![0x10, 0x20, 0x30]),
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
                "type": "spawn-app",
                "app": "1010101010101010101010101010101010101010",
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
    fn decode_receipt_spawn_app_receipt_error() {
        let logs = vec![Log {
            msg: b"Reached OOG".to_vec(),
            code: 0,
        }];

        let receipt = SpawnAppReceipt {
            version: 0,
            success: false,
            error: Some(ReceiptError::OOG),
            app_addr: None,
            init_state: None,
            returndata: None,
            gas_used: MaybeGas::with(1000),
            logs,
        };

        let bytes = crate::receipt::encode_app_receipt(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
               "type": "spawn-app",
               "success": false,
               "err_type": "oog",
               "logs": [{"code": 0, "msg": "Reached OOG"}],
            })
        );
    }

    #[test]
    fn decode_receipt_exec_app_receipt_success() {
        let state: State = [0xA0; 32].into();

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

        let receipt = ExecReceipt {
            version: 0,
            success: true,
            error: None,
            new_state: Some(state),
            returndata: Some(vec![0x10, 0x20]),
            gas_used: MaybeGas::with(10),
            logs,
        };

        let bytes = crate::receipt::encode_exec_receipt(&receipt);
        let data = json::bytes_to_str(&bytes);
        let json = decode_receipt(&json!({ "data": data })).unwrap();

        assert_eq!(
            json,
            json!({
                "success": true,
                "type": "exec-app",
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
