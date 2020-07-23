//!
//!   Receipt Error Encoding Format:
//!
//!   On failure (`is_success = 0`)
//!  +-------------------------------------------------------+
//!  |  tx type |    version   |  is_success |  error code   |
//!  | (1 byte) |  (1 nibble)  |  (1 nibble) |  (1 nibble)   |
//!  +__________|______________|_____________|_______________+
//!  |          |          |         |            |          |
//!  |  #logs   |  log #1  |  . . .  |  log #N-1  |  log #N  |
//!  +__________|__________|_________|____________|__________+
//!  |                                                       |
//!  |                     Error Blob                        |
//!  +_______________________________________________________+
//!
//!
//!  ## Error Blob
//!
//!  * OOG (Out-of-Gas) - no data
//!
//!  * Template Not Found
//!   +---------------------+
//!   |  Address (20 bytes) |
//!   +---------------------+
//!
//!  * App Not Found
//!   +-------------------+---------------+
//!   |  Template Address | App Address   |     
//!   |   (20 bytes)      |  (20 bytes)   |  
//!   +-------------------+---------------+
//!
//!  * Compilation Failed
//!   +-------------------+---------------+-----------------+
//!   |  Template Address | App Address   |     Error       |
//!   |   (20 bytes)      |  (20 bytes)   |  (UTF-8 String) |
//!   +-------------------+---------------------------------+
//!
//!  * Instantiation Failed
//!   +-------------------+---------------+-----------------+
//!   |  Template Address | App Address   |     Error       |
//!   |   (20 bytes)      |  (20 bytes)   |  (UTF-8 String) |
//!   +-------------------+---------------------------------+
//!
//!  * Function Not Found
//!   +-------------------+---------------+-----------------+
//!   |  Template Address | App Address   |    Func Index   |
//!   |   (20 bytes)      |  (20 bytes)   |   (varuint14)   |
//!   +-------------------+---------------------------------+
//!
//!  * Function Failed
//!   +-------------------+---------------+-------------------------------+
//!   |  Template Address |  App Address  |  Func Index  |     Error      |
//!   |   (20 bytes)      |   (20 bytes)  |  (varuint14) | (UTF-8 String) |
//!   +-------------------+-----------------------------------------------+
//!

use crate::api::raw::{self, Field};
use crate::helpers;

use crate::nib;
use crate::nibble::{Nibble, NibbleIter, NibbleWriter};

use svm_types::receipt::{Log, ReceiptError, ReceiptError as Err};
use svm_types::{AppAddr, TemplateAddr};

use super::logs;

pub(crate) fn encode_error(err: &ReceiptError, w: &mut NibbleWriter) {
    encode_err_type(err, w);
}

fn encode_err_type(err: &ReceiptError, w: &mut NibbleWriter) {
    let ty = match err {
        Err::OOG => 0,
        Err::TemplateNotFound(..) => 1,
        Err::AppNotFound(..) => 2,
        Err::CompilationFailed { .. } => 3,
        Err::InstantiationFailed { .. } => 4,
        Err::FuncNotFound { .. } => 5,
        Err::FuncFailed { .. } => 6,
    };

    w.push(nib!(ty));
}

pub(crate) fn decode_error(iter: &mut NibbleIter) -> (ReceiptError, Vec<Log>) {
    let err_type: Nibble = iter.next().unwrap();

    let logs = logs::decode_logs(iter);

    let err = {
        match err_type.inner() {
            0 => decode_oog(iter),
            1 => decode_template_not_found(iter),
            2 => decode_app_not_found(iter),
            3 => decode_compilation_err(iter),
            4 => decode_instantiation_err(iter),
            5 => decode_func_not_found(iter),
            6 => decode_func_err(iter),
            _ => unreachable!(),
        }
    };

    (err, logs)
}

fn decode_oog(iter: &mut NibbleIter) -> ReceiptError {
    ReceiptError::OOG
}

fn decode_template_not_found(iter: &mut NibbleIter) -> ReceiptError {
    let template_addr = helpers::decode_address(iter, Field::AppTemplate).unwrap();

    ReceiptError::TemplateNotFound(template_addr.into())
}

fn decode_app_not_found(iter: &mut NibbleIter) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(iter);

    ReceiptError::AppNotFound(template_addr, app_addr)
}

fn decode_compilation_err(iter: &mut NibbleIter) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(iter);
    let msg = decode_msg(iter);

    ReceiptError::CompilationFailed {
        template_addr,
        app_addr,
        msg,
    }
}

fn decode_instantiation_err(iter: &mut NibbleIter) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(iter);
    let msg = decode_msg(iter);

    ReceiptError::InstantiationFailed {
        template_addr,
        app_addr,
        msg,
    }
}

fn decode_func_not_found(iter: &mut NibbleIter) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(iter);
    let func_idx = decode_func_index(iter);
    let msg = decode_msg(iter);

    ReceiptError::FuncNotFound {
        template_addr,
        app_addr,
        func_idx,
    }
}

fn decode_func_err(iter: &mut NibbleIter) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(iter);
    let func_idx = decode_func_index(iter);
    let msg = decode_msg(iter);

    ReceiptError::FuncFailed {
        template_addr,
        app_addr,
        func_idx,
        msg,
    }
}

fn decode_addrs(iter: &mut NibbleIter) -> (TemplateAddr, AppAddr) {
    let template_addr = helpers::decode_address(iter, Field::AppTemplate).unwrap();
    let app_addr = helpers::decode_address(iter, Field::App).unwrap();

    (template_addr.into(), app_addr.into())
}

fn decode_msg(iter: &mut NibbleIter) -> String {
    helpers::decode_string(iter, Field::StringLength, Field::String).unwrap()
}

fn decode_func_index(iter: &mut NibbleIter) -> u16 {
    raw::decode_varuint14(iter, Field::FuncIndex).unwrap()
}
