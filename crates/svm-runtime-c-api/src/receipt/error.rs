//!
//!   Receipt Error Encoding Format:
//!
//!   On failure (`is_success = 0`)
//!  -----------------------------------------------------
//!  |   format   |                |                     |
//!  |  version   |  is_success    |     error size      |
//!  |  (4 bytes) |   (1 byte)     |      (2 bytes)      |
//!  |____________|________________|_____________________|
//!  |                                                   |
//!  |            error data (UTF-8 string)              |
//!  |___________________________________________________|
//!

use svm_runtime::receipt::Receipt;

use byteorder::{BigEndian, WriteBytesExt};

pub(crate) fn encode_error(buf: &mut Vec<u8>, receipt: &Receipt) {
    debug_assert!(receipt.is_success() == false);

    let error_str = receipt.error_string();
    let error_bytes = error_str.as_bytes();
    let error_size = error_bytes.len() as u16;

    buf.write_u16::<BigEndian>(error_size).unwrap();
    buf.extend_from_slice(error_bytes);
}
