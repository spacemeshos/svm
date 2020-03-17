//!     `Deploy Template` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  ----------------------------------------------------
//!  |   format   |              |                       |
//!  |  version   |  is_success  |    Template Address   |
//!  |  (4 bytes) |   (1 byte)   |      (20 bytes)       |
//!  |____________|______________|_______________________|
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use byteorder::{BigEndian, WriteBytesExt};

use svm_common::Address;
use svm_runtime::{
    error::DeployTemplateError,
    receipt::{Receipt, TemplateReceipt},
};

use super::{encode_error, helpers};

pub(crate) fn encode_template_receipt(receipt: &TemplateReceipt) -> Vec<u8> {
    let mut buf = Vec::new();

    let wrapped_receipt = Receipt::DeployTemplate(receipt);

    helpers::encode_is_success(&mut buf, &wrapped_receipt);

    if receipt.success {
        encode_template_addr(&mut buf, receipt);
    } else {
        encode_error(&mut buf, &wrapped_receipt);
    };

    buf
}

fn encode_template_addr(buf: &mut Vec<u8>, receipt: &TemplateReceipt) {
    debug_assert!(receipt.success);

    let addr = receipt.get_template_addr();

    buf.extend_from_slice(addr.inner().as_slice());
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_app::types::TemplateAddr;
    use svm_common::Address;
    use svm_runtime::receipt::TemplateReceipt;

    use crate::testing::{self, ClientTemplateReceipt};

    #[test]
    fn encode_deploy_deploy_receipt() {
        let addr: TemplateAddr = Address::of("my-template").into();

        let expected = ClientTemplateReceipt::Success { addr: addr.clone() };

        let receipt = TemplateReceipt {
            success: true,
            error: None,
            addr: Some(addr),
            gas_used: Some(100),
        };

        let bytes = encode_template_receipt(&receipt);
        let actual = testing::decode_template_receipt(&bytes[..]);

        assert_eq!(expected, actual);
    }
}
