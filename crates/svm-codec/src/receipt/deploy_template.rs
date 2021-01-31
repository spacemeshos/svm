//! `Deploy Template` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!
//!  +--------------------------------------:-----------------------------+
//!  | tx type  |   version   |  is_success | template address | gas_used |
//!  | (1 byte) |  (2 bytes)  |  (1 byte)   |    (20 bytes)    |          |
//!  +__________|_____________|_____________|__________________|__________+
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]
//!

use std::io::Cursor;

use svm_types::gas::MaybeGas;
use svm_types::receipt::{Receipt, TemplateReceipt};

use super::{decode_error, decode_receipt, encode_error, gas, logs, types};

use crate::common;
use crate::{ReadExt, WriteExt};

pub fn encode_template_receipt(receipt: &TemplateReceipt) -> Vec<u8> {
    let mut w = Vec::new();

    w.write_byte(types::DEPLOY_TEMPLATE);
    encode_version(receipt, &mut w);
    w.write_bool(receipt.success);

    if receipt.success {
        encode_template_addr(receipt, &mut w);
        gas::encode_gas_used(&receipt.gas_used, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = Vec::new();
        encode_error(receipt.get_error(), &logs, &mut w);
    };

    w
}

pub fn decode_template_receipt(bytes: &[u8]) -> TemplateReceipt {
    let mut cursor = Cursor::new(bytes);

    let ty = cursor.read_byte().unwrap();
    debug_assert_eq!(ty, types::DEPLOY_TEMPLATE);

    let version = common::decode_version(&mut cursor).unwrap();
    debug_assert_eq!(version, 0);

    let is_success = cursor.read_bool().unwrap();

    match is_success {
        false => {
            let (err, logs) = decode_error(&mut cursor);

            TemplateReceipt::from_err(err, logs)
        }
        true => {
            let addr = cursor.read_address().unwrap();
            let gas_used = gas::decode_gas_used(&mut cursor).unwrap();
            let logs = logs::decode_logs(&mut cursor).unwrap();

            TemplateReceipt {
                version,
                success: true,
                error: None,
                addr: Some(addr.into()),
                gas_used,
                logs,
            }
        }
        _ => unreachable!(),
    }
}

fn encode_version(receipt: &TemplateReceipt, w: &mut Vec<u8>) {
    let v = receipt.version;

    common::encode_version(v, w);
}

fn encode_template_addr(receipt: &TemplateReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let addr = receipt.get_template_addr();

    w.write_address(addr.inner());
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::gas::MaybeGas;
    use svm_types::receipt::TemplateReceipt;
    use svm_types::{Address, TemplateAddr};

    #[test]
    fn encode_decode_deploy_template_receipt() {
        let addr = Address::of("my-template").into();

        let receipt = TemplateReceipt {
            version: 0,
            success: true,
            error: None,
            addr: Some(addr),
            gas_used: MaybeGas::with(100),
            logs: Vec::new(),
        };

        let bytes = encode_template_receipt(&receipt);
        let decoded = decode_receipt(&bytes);

        assert_eq!(decoded.into_deploy_template(), receipt);
    }
}
