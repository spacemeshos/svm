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
//!  On failure (`is_success = 0`)
//!  -----------------------------------------------------
//!  |   format   |                |                     |
//!  |  version   |  is_success    |     error size      |
//!  |  (4 bytes) |   (1 byte)     |      (2 bytes)      |
//!  |____________|________________|_____________________|
//!  |                                                   |
//!  |            error data (UTF-8 string)              |
//!  |___________________________________________________|
//!

use byteorder::{BigEndian, WriteBytesExt};

use svm_common::Address;
use svm_runtime::{
    error::DeployTemplateError,
    receipt::{Receipt, TemplateReceipt},
};

use super::helpers;

pub(crate) fn encode_template_receipt(receipt: &TemplateReceipt) -> Vec<u8> {
    let mut buf = Vec::new();

    let wrapped_receipt = Receipt::DeployTemplate(receipt);

    helpers::encode_is_success(&mut buf, &wrapped_receipt);

    if receipt.success {
        encode_template_addr(&mut buf, receipt);
    } else {
        helpers::encode_error(&mut buf, &wrapped_receipt);
    };

    buf
}

fn encode_template_addr(buf: &mut Vec<u8>, receipt: &TemplateReceipt) {
    debug_assert!(receipt.success);

    let addr = receipt.get_template_addr();

    buf.extend_from_slice(addr.inner().as_slice());
}
