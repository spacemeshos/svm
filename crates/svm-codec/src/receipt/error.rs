//!
//!   Receipt Error Encoding Format:
//!
//!   On failure (`is_success = 0`)
//!  +-------------------------------------------------------+
//!  |  tx type |    version   |  is_success |  error code   |
//!  | (1 byte) |  (2 bytes)   |  (1 bytes)  |  (1 bytes)    |
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

use std::io::{Cursor, Read};

use crate::{Field, ReadExt, WriteExt};

use svm_types::receipt::{Log, ReceiptError, ReceiptError as Err};
use svm_types::{Address, AppAddr, TemplateAddr};

use super::logs;

pub(crate) fn encode_error(err: &ReceiptError, logs: &[Log], w: &mut Vec<u8>) {
    encode_err_type(err, w);

    logs::encode_logs(logs, w);

    match err {
        Err::OOG => (),
        Err::TemplateNotFound(template_addr) => w.write_address(template_addr.inner()),
        Err::AppNotFound(app_addr) => w.write_address(app_addr.inner()),
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
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(msg);
        }
        Err::FuncNotFound {
            app_addr,
            template_addr,
            func,
        } => {
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(func);
        }
        Err::FuncFailed {
            app_addr,
            template_addr,
            func,
            msg,
        } => {
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(func);
            w.write_string(msg);
        }
    };
}

fn encode_err_type(err: &ReceiptError, w: &mut Vec<u8>) {
    let ty = match err {
        Err::OOG => 0,
        Err::TemplateNotFound(..) => 1,
        Err::AppNotFound(..) => 2,
        Err::CompilationFailed { .. } => 3,
        Err::InstantiationFailed { .. } => 4,
        Err::FuncNotFound { .. } => 5,
        Err::FuncFailed { .. } => 6,
    };

    w.push(ty);
}

pub(crate) fn decode_error(cursor: &mut Cursor<&[u8]>) -> (ReceiptError, Vec<Log>) {
    let ty = cursor.read_byte().unwrap();
    let logs = logs::decode_logs(cursor).unwrap();

    let err = {
        match ty {
            0 => decode_oog(cursor),
            1 => decode_template_not_found(cursor),
            2 => decode_app_not_found(cursor),
            3 => decode_compilation_err(cursor),
            4 => decode_instantiation_err(cursor),
            5 => decode_func_not_found(cursor),
            6 => decode_func_err(cursor),
            _ => unreachable!(),
        }
    };

    (err, logs)
}

fn decode_oog(_cursor: &mut Cursor<&[u8]>) -> ReceiptError {
    ReceiptError::OOG
}

fn decode_template_not_found(cursor: &mut Cursor<&[u8]>) -> ReceiptError {
    let template_addr = decode_template_addr(cursor);

    ReceiptError::TemplateNotFound(template_addr.into())
}

fn decode_app_not_found(cursor: &mut Cursor<&[u8]>) -> ReceiptError {
    let app_addr = decode_app_addr(cursor);

    ReceiptError::AppNotFound(app_addr.into())
}

fn decode_compilation_err(cursor: &mut Cursor<&[u8]>) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let msg = decode_msg(cursor);

    ReceiptError::CompilationFailed {
        template_addr,
        app_addr,
        msg,
    }
}

fn decode_instantiation_err(cursor: &mut Cursor<&[u8]>) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let msg = decode_msg(cursor);

    ReceiptError::InstantiationFailed {
        template_addr,
        app_addr,
        msg,
    }
}

fn decode_func_not_found(cursor: &mut Cursor<&[u8]>) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let func = decode_func(cursor);

    ReceiptError::FuncNotFound {
        template_addr,
        app_addr,
        func,
    }
}

fn decode_func_err(cursor: &mut Cursor<&[u8]>) -> ReceiptError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let func = decode_func(cursor);
    let msg = decode_msg(cursor);

    ReceiptError::FuncFailed {
        template_addr,
        app_addr,
        func,
        msg,
    }
}

fn decode_func(cursor: &mut Cursor<&[u8]>) -> String {
    cursor.read_string().unwrap().unwrap()
}

fn decode_addrs(cursor: &mut Cursor<&[u8]>) -> (TemplateAddr, AppAddr) {
    let template_addr = decode_template_addr(cursor);
    let app_addr = decode_app_addr(cursor);

    (template_addr.into(), app_addr.into())
}

fn decode_template_addr(cursor: &mut Cursor<&[u8]>) -> Address {
    cursor.read_address().unwrap()
}

fn decode_app_addr(cursor: &mut Cursor<&[u8]>) -> Address {
    cursor.read_address().unwrap()
}

fn decode_msg(cursor: &mut Cursor<&[u8]>) -> String {
    cursor.read_string().unwrap().unwrap()
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

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let decoded = decode_error(&mut cursor);
    }

    #[test]
    fn decode_receipt_template_not_found() {
        let template_addr = Address::of("some-template");

        let err = ReceiptError::TemplateNotFound(template_addr.into());

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let decoded = decode_error(&mut cursor);
    }

    #[test]
    fn decode_receipt_app_not_found() {
        let app_addr = Address::of("some-app");

        let err = ReceiptError::AppNotFound(app_addr.into());

        let mut bytes = Vec::new();
        encode_error(&err, &test_logs(), &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);
        let decoded = decode_error(&mut cursor);
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

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let decoded = decode_error(&mut cursor);
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

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let decoded = decode_error(&mut cursor);
    }

    #[test]
    fn decode_receipt_func_not_found() {
        let template_addr = Address::of("some-template");
        let app_addr = Address::of("some-app");
        let func = "do_something".to_string();

        let err = ReceiptError::FuncNotFound {
            app_addr: app_addr.into(),
            template_addr: template_addr.into(),
            func,
        };

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let decoded = decode_error(&mut cursor);
    }

    #[test]
    fn decode_receipt_func_failed() {
        let template_addr = Address::of("some-template");
        let app_addr = Address::of("some-app");
        let func = "do_something".to_string();
        let msg = "Invalid input".to_string();

        let err = ReceiptError::FuncFailed {
            app_addr: app_addr.into(),
            template_addr: template_addr.into(),
            func,
            msg,
        };

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let decoded = decode_error(&mut cursor);
    }
}
