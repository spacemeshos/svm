//! `Deploy Template` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!
//!  +-----------------------------------------------------------------------+
//!  |          |             |             |                    |           |
//!  | tx type  |   version   |  is_success | template `Address` | gas_used  |
//!  | (1 byte) |  (2 bytes)  |  (1 byte)   |     (20 bytes)     | (8 bytes) |
//!  +__________|_____________|_____________|____________________|___________+
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]
//!

use std::io::Cursor;

use svm_types::DeployReceipt;

use super::{decode_error, encode_error, gas, logs, types};

use crate::version;
use crate::{ReadExt, WriteExt};

/// Encodes a [`DeployReceipt`] into its binary format.
pub fn encode_deploy_receipt(receipt: &DeployReceipt) -> Vec<u8> {
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

        encode_error(receipt.error(), &logs, &mut w);
    };

    w
}

/// Decodes a binary [`DeployReceipt`] transaction.
pub fn decode_deploy_receipt(bytes: &[u8]) -> DeployReceipt {
    let mut cursor = Cursor::new(bytes);

    let ty = cursor.read_byte().unwrap();
    debug_assert_eq!(ty, types::DEPLOY_TEMPLATE);

    let version = version::decode_version(&mut cursor).unwrap();
    debug_assert_eq!(version, 0);

    let is_success = cursor.read_bool().unwrap();

    match is_success {
        false => {
            let (err, logs) = decode_error(&mut cursor);

            DeployReceipt::from_err(err, logs)
        }
        true => {
            let addr = cursor.read_address().expect("expected an address");
            let gas_used = gas::decode_gas_used(&mut cursor).unwrap();
            let logs = logs::decode_logs(&mut cursor).unwrap();

            DeployReceipt {
                version,
                success: true,
                error: None,
                addr: Some(addr.into()),
                gas_used,
                logs,
            }
        }
    }
}

fn encode_version(receipt: &DeployReceipt, w: &mut Vec<u8>) {
    let v = receipt.version;

    version::encode_version(v, w);
}

fn encode_template_addr(receipt: &DeployReceipt, w: &mut Vec<u8>) {
    debug_assert!(receipt.success);

    let addr = receipt.template_addr();

    w.write_address(addr.inner());
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::{Address, DeployReceipt, Gas};

    use crate::receipt::decode_receipt;

    #[test]
    fn encode_decode_deploy_template_receipt() {
        let addr = Address::repeat(0xAB);

        let receipt = DeployReceipt {
            version: 0,
            success: true,
            error: None,
            addr: Some(addr.into()),
            gas_used: Gas::with(100),
            logs: Vec::new(),
        };

        let bytes = encode_deploy_receipt(&receipt);
        let decoded = decode_receipt(&bytes);

        assert_eq!(decoded.into_deploy(), receipt);
    }
}
