//! ## Error Receipt Binary Format Version 0
//!
//!  On failure (`is_success = 0`)
//!   
//!  ```text
//!  +-------------------------------------------------------+
//!  |          |              |             |               |
//!  |  tx type |   version    |  is_success |  error code   |
//!  | (1 byte) |  (2 bytes)   |   (1 byte)  |   (1 byte)    |
//!  |          |              |             |               |  
//!  +-------------------------------------------------------+
//!  |          |          |         |            |          |
//!  |  #logs   |  log #1  |  . . .  |  log #N-1  |  log #N  |
//!  | (1 byte) |          |         |            |          |
//!  |          |          |         |            |          |
//!  +-------------------------------------------------------+
//!  |                                                       |
//!  |                     Error Blob                        |
//!  |                                                       |
//!  +-------------------------------------------------------+
//!  ```
//!
//!
//!  ### Error Blob
//!
//!  ### Important:
//!
//!  Each `Error Message` Field is truncated to fit into at most 255 bytes.
//!
//!  * OOG (Out-of-Gas) - no data
//!
//!  * Template Not Found
//!   +---------------------+
//!   |  Template Address   |
//!   |     (20 bytes)      |
//!   +---------------------+
//!
//!  * Account Not Found
//!   +---------------------+
//!   |   Account Address   |
//!   |     (20 bytes)      |
//!   +---------------------+
//!
//!  * Compilation Failed
//!   +-------------------+-----------------+-----------------+
//!   |  Template Address | Account Address |     Message     |
//!   |   (20 bytes)      |  (20 bytes)     |  (UTF-8 String) |
//!   +-------------------+-----------------+-----------------+
//!
//!  * Instantiation Failed
//!   +-------------------+-----------------+-----------------+
//!   |  Template Address | Account Address |     Message     |
//!   |   (20 bytes)      |  (20 bytes)     |  (UTF-8 String) |
//!   +-------------------+-----------------+-----------------+
//!
//!  * Function Not Found
//!   +-------------------+-----------------+--------------+
//!   |  Template Address | Account Address |   Function   |
//!   |   (20 bytes)      |  (20 bytes)     |   (String)   |
//!   +-------------------+-----------------+--------------+
//!
//!  * Function Failed
//!   +-------------------+------------------+------------+----------------+
//!   |  Template Address |  Account Address |  Function  |    Message     |
//!   |   (20 bytes)      |   (20 bytes)     |  (String)  | (UTF-8 String) |
//!   +-------------------+------------------+------------+----------------+
//!
//!  * Function Not Allowed
//!   +-------------------+-------------------+------------+----------------+
//!   |  Template Address |  Account Address  |  Function  |    Message     |
//!   |   (20 bytes)      |   (20 bytes)      |  (String)  | (UTF-8 String) |
//!   +-------------------+-------------------+------------+----------------+
//!
//!  * Function Invalid Signature
//!   +-------------------+-------------------+------------+
//!   |  Template Address |  Account Address  |  Function  |     
//!   |   (20 bytes)      |   (20 bytes)      |  (String)  |
//!   +-------------------+-------------------+------------+
//!

use std::io::Cursor;

use svm_types::{Address, ReceiptLog, RuntimeError, TemplateAddr};

use super::logs;
use crate::{ReadExt, WriteExt};

pub(crate) fn encode_error(err: &RuntimeError, logs: &[ReceiptLog], w: &mut Vec<u8>) {
    encode_err_type(err, w);

    logs::encode_logs(logs, w);

    match err {
        RuntimeError::OOG => (),
        RuntimeError::TemplateNotFound(template) => encode_template(template, w),
        RuntimeError::AccountNotFound(target) => encode_target(target, w),
        RuntimeError::CompilationFailed {
            target,
            template,
            msg,
        }
        | RuntimeError::InstantiationFailed {
            target,
            template,
            msg,
        } => {
            encode_template(template, w);
            encode_target(target, w);
            encode_msg(msg, w);
        }
        RuntimeError::FuncNotFound {
            target,
            template,
            func,
        } => {
            encode_template(template, w);
            encode_target(target, w);
            encode_func(func, w);
        }
        RuntimeError::FuncFailed {
            target,
            template,
            func,
            msg,
        } => {
            encode_template(template, w);
            encode_target(target, w);
            encode_func(func, w);
            encode_msg(msg, w);
        }
        RuntimeError::FuncNotAllowed {
            target,
            template,
            func,
            msg,
        } => {
            encode_template(template, w);
            encode_target(target, w);
            encode_func(func, w);
            encode_msg(msg, w);
        }
        RuntimeError::FuncInvalidSignature {
            target,
            template,
            func,
        } => {
            encode_template(template, w);
            encode_target(target, w);
            encode_func(func, w);
        }
    };
}

fn encode_template(template: &TemplateAddr, w: &mut Vec<u8>) {
    w.write_template_addr(template);
}

fn encode_target(target: &Address, w: &mut Vec<u8>) {
    w.write_address(target);
}

fn encode_func(func: &str, w: &mut Vec<u8>) {
    w.write_string(func);
}

fn encode_msg(msg: &str, w: &mut Vec<u8>) {
    if msg.len() > 255 {
        let bytes = &msg.as_bytes()[0..255];
        let msg = unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
        w.write_string(&msg);
    } else {
        w.write_string(msg);
    }
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
            0 => oog(cursor),
            1 => template_not_found(cursor),
            2 => account_not_found(cursor),
            3 => compilation_error(cursor),
            4 => instantiation_error(cursor),
            5 => func_not_found(cursor),
            6 => func_failed(cursor),
            7 => func_not_allowed(cursor),
            8 => func_invalid_sig(cursor),
            _ => unreachable!(),
        }
    };

    (err, logs)
}

fn oog(_cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    RuntimeError::OOG
}

fn template_not_found(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    RuntimeError::TemplateNotFound(template_addr)
}

fn account_not_found(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let account = decode_account_addr(cursor);
    RuntimeError::AccountNotFound(account.into())
}

fn compilation_error(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::CompilationFailed {
        template: template_addr,
        target: account_addr,
        msg,
    }
}

fn instantiation_error(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::InstantiationFailed {
        template: template_addr,
        target: account_addr,
        msg,
    }
}

fn func_not_found(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let func = decode_func(cursor);

    RuntimeError::FuncNotFound {
        template: template_addr,
        target: account_addr,
        func,
    }
}

fn func_failed(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let func = decode_func(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::FuncFailed {
        template: template_addr,
        target: account_addr,
        func,
        msg,
    }
}

fn func_not_allowed(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let func = decode_func(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::FuncNotAllowed {
        template: template_addr,
        target: account_addr,
        func,
        msg,
    }
}

fn func_invalid_sig(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let func = decode_func(cursor);

    RuntimeError::FuncInvalidSignature {
        template: template_addr,
        target: account_addr,
        func,
    }
}

fn decode_func(cursor: &mut Cursor<&[u8]>) -> String {
    cursor.read_string().unwrap().unwrap()
}

fn decode_template_addr(cursor: &mut Cursor<&[u8]>) -> TemplateAddr {
    cursor.read_template_addr().unwrap()
}

fn decode_account_addr(cursor: &mut Cursor<&[u8]>) -> Address {
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
        let template_addr = TemplateAddr::of("@Template");
        let err = RuntimeError::TemplateNotFound(template_addr);

        let mut buf = Vec::new();
        encode_error(&err, &test_logs(), &mut buf);

        let mut cursor = Cursor::new(&buf[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_account_not_found() {
        let account_addr = Address::of("@Account");
        let err = RuntimeError::AccountNotFound(account_addr);

        let mut bytes = Vec::new();
        encode_error(&err, &test_logs(), &mut bytes);

        let mut cursor = Cursor::new(&bytes[..]);

        let (decoded, logs) = decode_error(&mut cursor);

        assert_eq!(decoded, err);
        assert_eq!(logs, test_logs());
    }

    #[test]
    fn decode_receipt_compilation_failed() {
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");

        let err = RuntimeError::CompilationFailed {
            target: account_addr,
            template: template_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");

        let err = RuntimeError::InstantiationFailed {
            target: account_addr,
            template: template_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "do_something".to_string();

        let err = RuntimeError::FuncNotFound {
            target: account_addr,
            template: template_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "do_something".to_string();
        let msg = "Invalid input".to_string();

        let err = RuntimeError::FuncFailed {
            target: account_addr,
            template: template_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "init".to_string();
        let msg = "expected a ctor".to_string();

        let err = RuntimeError::FuncNotAllowed {
            target: account_addr,
            template: template_addr,
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
