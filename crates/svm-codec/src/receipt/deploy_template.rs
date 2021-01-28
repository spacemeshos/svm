//! `Deploy Template` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!
//!  +-------------------------------------------------------------------+
//!  | tx type  |   version  |  is_success | Template Address | gas_used |
//!  | (1 byte) | (1 nibble) |  (1 byte)   |    (20 bytes)    |          |
//!  +_______________________|_____________|__________________|__________+
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]
//!

use std::io::Cursor;

use svm_types::gas::MaybeGas;
use svm_types::receipt::{Receipt, TemplateReceipt};

use super::{decode_error, encode_error, gas, logs};

use crate::version;
use crate::{ReadExt, WriteExt};

pub fn encode_template_receipt(receipt: &TemplateReceipt) -> Vec<u8> {
    let mut w = Vec::new();

    w.push(super::types::DEPLOY_TEMPLATE);
    version::encode_version(0, &mut w);
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
    debug_assert_eq!(ty, crate::receipt::types::DEPLOY_TEMPLATE);

    let version = version::decode_version(&mut cursor).unwrap();
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
            success: true,
            error: None,
            addr: Some(addr),
            gas_used: MaybeGas::with(100),
            logs: Vec::new(),
        };

        let bytes = encode_template_receipt(&receipt);
        let decoded = crate::receipt::decode_receipt(&bytes);

        assert_eq!(decoded.into_deploy_template(), receipt);
    }
}
