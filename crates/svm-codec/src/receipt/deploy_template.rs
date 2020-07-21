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

use crate::api::raw;
use crate::nibble::{NibbleIter, NibbleWriter};

use svm_types::gas::MaybeGas;
use svm_types::receipt::{Receipt, TemplateReceipt};

use super::{encode_error, helpers};

pub fn encode_template_receipt(receipt: &TemplateReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::DeployTemplate(receipt);

    helpers::encode_type(super::types::DEPLOY_TEMPLATE, &mut w);
    raw::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_template_addr(receipt, &mut w);
        helpers::encode_gas_used(&wrapped_receipt, &mut w);
    } else {
        encode_error(&wrapped_receipt, &mut w);
    };

    w.into_bytes()
}

pub fn decode_template_receipt(bytes: &[u8]) -> TemplateReceipt {
    let mut iter = NibbleIter::new(bytes);

    let ty = helpers::decode_type(&mut iter);
    debug_assert_eq!(ty, crate::receipt::types::DEPLOY_TEMPLATE);

    let version = raw::decode_version(&mut iter).unwrap();
    debug_assert_eq!(version, 0);

    let is_success = helpers::decode_is_success(&mut iter);

    match is_success {
        0 => {
            // error
            let error = helpers::decode_receipt_error(&mut iter);

            todo!()
            // ClientTemplateReceipt::Failure { error }
        }
        1 => {
            // success
            let addr = helpers::decode_address(&mut iter);
            let gas_used = helpers::decode_gas_used(&mut iter);

            TemplateReceipt {
                success: true,
                error: None,
                addr: Some(addr.into()),
                gas_used: MaybeGas::with(gas_used),
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

    use crate::receipt::testing::{self, ClientReceipt, ClientTemplateReceipt};

    #[test]
    fn encode_deploy_deploy_receipt() {
        let addr: TemplateAddr = Address::of("my-template").into();

        let expected = ClientReceipt::DeployTemplate(ClientTemplateReceipt::Success {
            addr: addr.clone(),
            gas_used: 100,
        });

        let receipt = TemplateReceipt {
            success: true,
            error: None,
            addr: Some(addr),
            gas_used: MaybeGas::with(100),
        };

        let bytes = encode_template_receipt(&receipt);
        let actual = testing::decode_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
