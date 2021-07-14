//!
//!   Receipt Error Encoding Format:
//!
//!   On failure (`is_success = 0`)
//!  +-------------------------------------------------------+
//!  |  tx type |   version    |  is_success |  error code   |
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
//!   +-------------------+---------------+--------------+
//!   |  Template Address | App Address   |   Function   |
//!   |   (20 bytes)      |  (20 bytes)   |   (String)   |
//!   +-------------------+------------------------------+
//!
//!  * Function Failed
//!   +-------------------+---------------+-----------------------------+
//!   |  Template Address |  App Address  |  Function  |     Error      |
//!   |   (20 bytes)      |   (20 bytes)  |   (String) | (UTF-8 String) |
//!   +-------------------+---------------------------------------------+
//!
//!  * Function Not Allowed
//!   +-------------------+---------------+-----------------------------+
//!   |  Template Address |  App Address  |  Function  |     Error      |
//!   |   (20 bytes)      |   (20 bytes)  |   (String) | (UTF-8 String) |
//!   +-------------------+---------------------------------------------+
//!
//!  * Function Invalid Signature
//!   +-------------------+---------------+------------+
//!   |  Template Address |  App Address  |  Function  |     
//!   |   (20 bytes)      |   (20 bytes)  |   (String) |
//!   +-------------------+----------------------------+
//!

use svm_types::{AccountAddr, Address, ReceiptLog, RuntimeError, TemplateAddr};

use std::io::Cursor;

use super::logs;
use crate::{ReadExt, WriteExt};

pub(crate) fn encode_error(err: &RuntimeError, logs: &[ReceiptLog], w: &mut Vec<u8>) {
    encode_err_type(err, w);

    logs::encode_logs(logs, w);

    match err {
        RuntimeError::OOG => (),
        RuntimeError::TemplateNotFound(template_addr) => w.write_address(template_addr.inner()),
        RuntimeError::AccountNotFound(app_addr) => w.write_address(app_addr.inner()),
        RuntimeError::CompilationFailed {
            account_addr: app_addr,
            template_addr,
            msg,
        }
        | RuntimeError::InstantiationFailed {
            account_addr: app_addr,
            template_addr,
            msg,
        } => {
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(msg);
        }
        RuntimeError::FuncNotFound {
            accunt_addr: app_addr,
            template_addr,
            func,
        } => {
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(func);
        }
        RuntimeError::FuncFailed {
            account_addr: app_addr,
            template_addr,
            func,
            msg,
        } => {
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(func);
            w.write_string(msg);
        }
        RuntimeError::FuncNotAllowed {
            account_addr: app_addr,
            template_addr,
            func,
            msg,
        } => {
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(func);
            w.write_string(msg);
        }
        RuntimeError::FuncInvalidSignature {
            account_addr: app_addr,
            template_addr,
            func,
        } => {
            w.write_address(template_addr.inner());
            w.write_address(app_addr.inner());
            w.write_string(func);
        }
    };
}

fn encode_err_type(err: &RuntimeError, w: &mut Vec<u8>) {
    let ty = match err {
        RuntimeError::OOG => 0,
        RuntimeError::TemplateNotFound(..) => 1,
        RuntimeError::AccountNotFound(..) => 2,
        RuntimeError::CompilationFailed { .. } => 3,
        RuntimeError::InstantiationFailed { .. } => 4,
        RuntimeError::FuncNotFound { .. } => 5,
        RuntimeError::FuncFailed { .. } => 6,
        RuntimeError::FuncNotAllowed { .. } => 7,
        RuntimeError::FuncInvalidSignature { .. } => 8,
    };

    w.push(ty);
}

pub(crate) fn decode_error(cursor: &mut Cursor<&[u8]>) -> (RuntimeError, Vec<ReceiptLog>) {
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
            6 => decode_func_failed(cursor),
            7 => decode_func_not_allowed(cursor),
            8 => decode_func_invalid_sig(cursor),
            _ => unreachable!(),
        }
    };

    (err, logs)
}

fn decode_oog(_cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    RuntimeError::OOG
}

fn decode_template_not_found(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);

    RuntimeError::TemplateNotFound(template_addr.into())
}

fn decode_app_not_found(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let app_addr = decode_app_addr(cursor);

    RuntimeError::AccountNotFound(app_addr.into())
}

fn decode_compilation_err(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::CompilationFailed {
        template_addr,
        account_addr: app_addr,
        msg,
    }
}

fn decode_instantiation_err(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::InstantiationFailed {
        template_addr,
        account_addr: app_addr,
        msg,
    }
}

fn decode_func_not_found(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let func = decode_func(cursor);

    RuntimeError::FuncNotFound {
        template_addr,
        accunt_addr: app_addr,
        func,
    }
}

fn decode_func_failed(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let func = decode_func(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::FuncFailed {
        template_addr,
        account_addr: app_addr,
        func,
        msg,
    }
}

fn decode_func_not_allowed(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let func = decode_func(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::FuncNotAllowed {
        template_addr,
        account_addr: app_addr,
        func,
        msg,
    }
}

fn decode_func_invalid_sig(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let (template_addr, app_addr) = decode_addrs(cursor);
    let func = decode_func(cursor);

    RuntimeError::FuncInvalidSignature {
        template_addr,
        account_addr: app_addr,
        func,
    }
}

fn decode_func(cursor: &mut Cursor<&[u8]>) -> String {
    cursor.read_string().unwrap().unwrap()
}

fn decode_addrs(cursor: &mut Cursor<&[u8]>) -> (TemplateAddr, AccountAddr) {
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

    fn test_logs() -> Vec<ReceiptLog> {
        vec![
            ReceiptLog {
                msg: b"Log entry #1".to_vec(),
                code: 0,
            },
            ReceiptLog {
                msg: b"Log entry #2".to_vec(),
                code: 1,
            },
        ]
    }

    #[test]
    fn decode_receipt_oog() {
        let err = RuntimeError::OOG;

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let _decoded = decode_error(&mut cursor);
    }

    #[test]
    fn decode_receipt_template_not_found() {
        let template_addr = Address::of("some-template");

        let err = RuntimeError::TemplateNotFound(template_addr.into());

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_app_not_found() {
        let app_addr = Address::of("some-app");

        let err = RuntimeError::AccountNotFound(app_addr.into());

        let mut bytes = Vec::new();
        encode_error(&err, &test_logs(), &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_compilation_failed() {
        let template_addr = Address::of("some-template").into();
        let app_addr = Address::of("some-app").into();

        let err = RuntimeError::CompilationFailed {
            account_addr: app_addr,
            template_addr,
            msg: "Invalid code".to_string(),
        };

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_instantiation_failed() {
        let template_addr = Address::of("some-template").into();
        let app_addr = Address::of("some-app").into();

        let err = RuntimeError::InstantiationFailed {
            account_addr: app_addr,
            template_addr,
            msg: "Invalid input".to_string(),
        };

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_func_not_found() {
        let template_addr = Address::of("some-template").into();
        let app_addr = Address::of("some-app").into();
        let func = "do_something".to_string();

        let err = RuntimeError::FuncNotFound {
            accunt_addr: app_addr,
            template_addr,
            func,
        };

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_func_failed() {
        let template_addr = Address::of("some-template").into();
        let app_addr = Address::of("some-app").into();
        let func = "do_something".to_string();
        let msg = "Invalid input".to_string();

        let err = RuntimeError::FuncFailed {
            account_addr: app_addr,
            template_addr,
            func,
            msg,
        };

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_func_not_allowed() {
        let template_addr = Address::of("some-template").into();
        let app_addr = Address::of("some-app").into();
        let func = "init".to_string();
        let msg = "expected a ctor".to_string();

        let err = RuntimeError::FuncNotAllowed {
            account_addr: app_addr,
            template_addr,
            func,
            msg,
        };

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }
}
