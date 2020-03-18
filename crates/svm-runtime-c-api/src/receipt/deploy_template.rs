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

use svm_app::raw::NibbleWriter;
use svm_common::Address;
use svm_runtime::{
    error::DeployTemplateError,
    receipt::{Receipt, TemplateReceipt},
};

use super::{encode_error, helpers};

pub(crate) fn encode_template_receipt(receipt: &TemplateReceipt) -> Vec<u8> {
    let mut w = NibbleWriter::new();

    let wrapped_receipt = Receipt::DeployTemplate(receipt);

    helpers::encode_version(0, &mut w);
    helpers::encode_is_success(&wrapped_receipt, &mut w);

    if receipt.success {
        encode_template_addr(receipt, &mut w);
    } else {
        encode_error(&wrapped_receipt, &mut w);
    };

    w.into_bytes()
}

fn encode_template_addr(receipt: &TemplateReceipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.success);

    let addr = receipt.get_template_addr();
    helpers::encode_addr(addr.inner(), w);
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
