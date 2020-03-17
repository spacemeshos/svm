//!
//!           `Spawn App` Receipt Raw Format Version 0
//!
//!  On success (`is_success = 1`)
//!  +-------------------------------------------------------+
//!  |   format   |              |                           |
//!  |  version   |  is_success  |     App Address           |
//!  |  (4 bytes) |   (1 byte)   |      (20 bytes)           |
//!  +____________|______________|___________________________+
//!  |              |           |          |        |        |
//!  |  init state  | #returns  | ret #1   | ret #1 |        |
//!  |  (32 bytes)  | (2 bytes) |   type   |  value |  ...   |
//!  |              |           | (1 byte) |        |        |
//!  +______________|___________|__________|________|________+
//!
//!
//!  On success (`is_success = 0`)
//!  See [error.rs][./error.rs]

use byteorder::{BigEndian, WriteBytesExt};

use svm_common::Address;
use svm_runtime::{
    error::DeployTemplateError,
    receipt::{Receipt, SpawnAppReceipt},
};

use super::{encode_error, helpers};

pub(crate) fn encode_app_receipt(receipt: &SpawnAppReceipt) -> Vec<u8> {
    let mut buf = Vec::new();

    let wrapped_receipt = Receipt::SpawnApp(receipt);

    helpers::encode_is_success(&mut buf, &wrapped_receipt);

    if receipt.success {
        encode_app_addr(&mut buf, receipt);
        encode_init_state(&mut buf, receipt);
        helpers::encode_returns(&mut buf, &wrapped_receipt);
    } else {
        encode_error(&mut buf, &wrapped_receipt);
    };

    buf
}

fn encode_app_addr(buf: &mut Vec<u8>, receipt: &SpawnAppReceipt) {
    debug_assert!(receipt.success);

    let addr = receipt.get_app_addr();

    buf.extend_from_slice(addr.inner().as_slice());
}

fn encode_init_state(buf: &mut Vec<u8>, receipt: &SpawnAppReceipt) {
    debug_assert!(receipt.success);

    let state = receipt.get_init_state();

    buf.extend_from_slice(state.as_slice());
}
