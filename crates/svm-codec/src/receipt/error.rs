//!
//!   Receipt Error Encoding Format:
//!
//!   On failure (`is_success = 0`)
//!  +---------------------------------------------------+
//!  |            |                |                     |
//!  |  version   |  is_success    |     error size      |
//!  |            |   (1 nibble)   |     (varuint14)     |
//!  +____________|________________|_____________________+
//!  |                                                   |
//!  |            error data (UTF-8 string)              |
//!  +___________________________________________________+
//!

use crate::api::raw::encode_varuint14;
use crate::nibble::NibbleWriter;

use svm_types::receipt::Receipt;

pub(crate) fn encode_error(receipt: &Receipt, w: &mut NibbleWriter) {
    debug_assert!(receipt.is_success() == false);

    let error_str = receipt.error_string();
    let error_bytes = error_str.as_bytes();
    let error_size = error_bytes.len() as u16;

    encode_varuint14(error_size, w);

    w.write_bytes(error_bytes)
}
