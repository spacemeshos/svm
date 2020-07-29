//!     `Deploy Template` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!
//!  +-------------------------------------------------------------------+
//!  | tx type  |   version  |  is_success | Template Address | gas_used |
//!  | (1 byte) | (1 nibble) |  (1 byte)   |    (20 bytes)    |          |
//!  +_______________________|___________________________________________+
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]
//!

use crate::nibble::{NibbleIter, NibbleWriter};

use svm_types::gas::MaybeGas;
use svm_types::receipt::{Receipt, TemplateReceipt};

use super::{decode_error, encode_error, helpers, logs};

pub fn encode_template_receipt(receipt: &TemplateReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::DeployTemplate(receipt);

    helpers::encode_type(super::types::DEPLOY_TEMPLATE, &mut w);
    helpers::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_template_addr(receipt, &mut w);
        helpers::encode_gas_used(&wrapped_receipt, &mut w);
        logs::encode_logs(&receipt.logs, &mut w);
    } else {
        let logs = Vec::new();
        encode_error(receipt.get_error(), &logs, &mut w);
    };

    w.into_bytes()
}

pub fn decode_template_receipt(bytes: &[u8]) -> TemplateReceipt {
    let mut iter = NibbleIter::new(bytes);

    let ty = helpers::decode_type(&mut iter);
    debug_assert_eq!(ty, crate::receipt::types::DEPLOY_TEMPLATE);

    let version = helpers::decode_version(&mut iter).unwrap();
    debug_assert_eq!(version, 0);

    let is_success = helpers::decode_is_success(&mut iter);

    match is_success {
        0 => {
            // error
            let (err, logs) = decode_error(&mut iter);
            TemplateReceipt::from_err(err, logs)
        }
        1 => {
            // success
            let addr = helpers::decode_address(&mut iter);
            let gas_used = helpers::decode_gas_used(&mut iter);
            let logs = logs::decode_logs(&mut iter);

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

fn encode_template_addr(receipt: &TemplateReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let addr = receipt.get_template_addr();
    helpers::encode_addr(addr.inner(), w);
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::{gas::MaybeGas, receipt::TemplateReceipt, Address, TemplateAddr};

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
