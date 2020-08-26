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
//!   |  Template Address   |
//!   |     (20 bytes)      |
//!   +---------------------+
//!
//!  * App Not Found
//!   +---------------------+
//!   |     App Address     |
//!   |     (20 bytes)      |
//!   +---------------------+
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

use crate::api::raw::Field;
use crate::helpers;

use svm_nibble::{nib, Nibble, NibbleIter, NibbleWriter};

use svm_types::receipt::{Log, ReceiptError, ReceiptError as Err};
use svm_types::{Address, AppAddr, TemplateAddr};

use super::logs;

pub(crate) fn encode_error(err: &ReceiptError, logs: &[Log], w: &mut NibbleWriter) {
    encode_err_type(err, w);

    logs::encode_logs(logs, w);

    match err {
        Err::OOG => (),
        Err::TemplateNotFound(template_addr) => helpers::encode_address(template_addr.inner(), w),
        Err::AppNotFound(app_addr) => helpers::encode_address(app_addr.inner(), w),
        Err::CompilationFailed {
            app_addr,
            template_addr,
            msg,
        }
        | Err::InstantiationFailed {
            app_addr,
            template_addr,
            msg,
        } => {
            helpers::encode_address(template_addr.inner(), w);
            helpers::encode_address(app_addr.inner(), w);
            helpers::encode_string(msg, w);
        }
        Err::FuncNotFound {
            app_addr,
            template_addr,
            func,
        } => {
            helpers::encode_address(template_addr.inner(), w);
            helpers::encode_address(app_addr.inner(), w);
            helpers::encode_string(func, w);
        }
        Err::FuncFailed {
            app_addr,
            template_addr,
            func,
            msg,
        } => {
            helpers::encode_address(template_addr.inner(), w);
            helpers::encode_address(app_addr.inner(), w);
            helpers::encode_string(func, w);
            helpers::encode_string(msg, w);
        }
    };
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
    let template_addr = decode_template_addr(iter);

    ReceiptError::TemplateNotFound(template_addr.into())
}

fn decode_app_not_found(iter: &mut NibbleIter) -> ReceiptError {
    let app_addr = decode_app_addr(iter);

    ReceiptError::AppNotFound(app_addr.into())
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
    let func = helpers::decode_string(iter, Field::FuncNameLength, Field::FuncName).unwrap();

    ReceiptError::FuncNotFound {
        template_addr,
        app_addr,
        func,
    }
}

fn decode_func_err(iter: &mut NibbleIter) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(iter);
    let func = helpers::decode_string(iter, Field::FuncNameLength, Field::FuncName).unwrap();
    let msg = decode_msg(iter);

    ReceiptError::FuncFailed {
        template_addr,
        app_addr,
        func,
        msg,
    }
}

fn decode_addrs(iter: &mut NibbleIter) -> (TemplateAddr, AppAddr) {
    let template_addr = decode_template_addr(iter);
    let app_addr = decode_app_addr(iter);

    (template_addr.into(), app_addr.into())
}

fn decode_template_addr(iter: &mut NibbleIter) -> Address {
    helpers::decode_address(iter, Field::TemplateAddr).unwrap()
}

fn decode_app_addr(iter: &mut NibbleIter) -> Address {
    helpers::decode_address(iter, Field::AppAddr).unwrap()
}

fn decode_msg(iter: &mut NibbleIter) -> String {
    helpers::decode_string(iter, Field::StringLength, Field::String).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use svm_types::Address;

    fn test_logs() -> Vec<Log> {
        vec![
            Log {
                msg: b"Log entry #1".to_vec(),
                code: 0,
            },
            Log {
                msg: b"Log entry #2".to_vec(),
                code: 1,
            },
        ]
    }

    #[test]
    fn decode_receipt_oog() {
        let err = ReceiptError::OOG;

        let mut w = NibbleWriter::new();
        encode_error(&err, &test_logs(), &mut w);
        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_error(&mut iter);
    }

    #[test]
    fn decode_receipt_template_not_found() {
        let template_addr = Address::of("some-template");

        let err = ReceiptError::TemplateNotFound(template_addr.into());

        let mut w = NibbleWriter::new();
        encode_error(&err, &test_logs(), &mut w);
        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_error(&mut iter);
    }

    #[test]
    fn decode_receipt_app_not_found() {
        let app_addr = Address::of("some-app");

        let err = ReceiptError::AppNotFound(app_addr.into());

        let mut w = NibbleWriter::new();
        encode_error(&err, &test_logs(), &mut w);
        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_error(&mut iter);
    }

    #[test]
    fn decode_receipt_compilation_failed() {
        let template_addr = Address::of("some-template");
        let app_addr = Address::of("some-app");

        let err = ReceiptError::CompilationFailed {
            app_addr: app_addr.into(),
            template_addr: template_addr.into(),
            msg: "Invalid code".to_string(),
        };

        let mut w = NibbleWriter::new();
        encode_error(&err, &test_logs(), &mut w);
        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_error(&mut iter);
    }

    #[test]
    fn decode_receipt_instantiation_failed() {
        let template_addr = Address::of("some-template");
        let app_addr = Address::of("some-app");

        let err = ReceiptError::InstantiationFailed {
            app_addr: app_addr.into(),
            template_addr: template_addr.into(),
            msg: "Invalid input".to_string(),
        };

        let mut w = NibbleWriter::new();
        encode_error(&err, &test_logs(), &mut w);
        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_error(&mut iter);
    }

    #[test]
    fn decode_receipt_func_not_found() {
        let template_addr = Address::of("some-template");
        let app_addr = Address::of("some-app");
        let func = 1;

        let err = ReceiptError::FuncNotFound {
            app_addr: app_addr.into(),
            template_addr: template_addr.into(),
            func,
        };

        let mut w = NibbleWriter::new();
        encode_error(&err, &test_logs(), &mut w);
        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_error(&mut iter);
    }

    #[test]
    fn decode_receipt_func_failed() {
        let template_addr = Address::of("some-template");
        let app_addr = Address::of("some-app");
        let func = 1;
        let msg = "Invalid input".to_string();

        let err = ReceiptError::FuncFailed {
            app_addr: app_addr.into(),
            template_addr: template_addr.into(),
            func,
            msg,
        };

        let mut w = NibbleWriter::new();
        encode_error(&err, &test_logs(), &mut w);
        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let decoded = decode_error(&mut iter);
    }
}
