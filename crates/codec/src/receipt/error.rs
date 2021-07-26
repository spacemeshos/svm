//!
//!   Receipt Error Encoding Format:
//!
//!   On failure (`is_success = 0`)
//!   
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
//!  * Account Not Found
//!   +---------------------+
//!   |   Account Address   |
//!   |     (20 bytes)      |
//!   +---------------------+
//!
//!  * Compilation Failed
//!   +-------------------+-----------------+-----------------+
//!   |  Template Address | Account Address |     Error       |
//!   |   (20 bytes)      |  (20 bytes)     |  (UTF-8 String) |
//!   +-------------------+-----------------+-----------------+
//!
//!  * Instantiation Failed
//!   +-------------------+-----------------+-----------------+
//!   |  Template Address | Account Address |     Error       |
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
//!   +-------------------+------------------+-----------------------------+
//!   |  Template Address |  Account Address |  Function  |     Error      |
//!   |   (20 bytes)      |   (20 bytes)     |  (String)  | (UTF-8 String) |
//!   +-------------------+------------------+------------+----------------+
//!
//!  * Function Not Allowed
//!   +-------------------+-------------------+-----------------------------+
//!   |  Template Address |  Account Address  |  Function  |     Error      |
//!   |   (20 bytes)      |   (20 bytes)      |   (String) | (UTF-8 String) |
//!   +-------------------+-------------------+------------+----------------+
//!
//!  * Function Invalid Signature
//!   +-------------------+-------------------+------------+
//!   |  Template Address |  Account Address  |  Function  |     
//!   |   (20 bytes)      |   (20 bytes)      |   (String) |
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
        RuntimeError::TemplateNotFound(template_addr) => w.write_template_addr(template_addr),
        RuntimeError::AccountNotFound(account_addr) => w.write_address(account_addr),
        RuntimeError::CompilationFailed {
            account_addr,
            template_addr,
            msg,
        }
        | RuntimeError::InstantiationFailed {
            account_addr,
            template_addr,
            msg,
        } => {
            w.write_template_addr(template_addr);
            w.write_address(account_addr);
            w.write_string(msg);
        }
        RuntimeError::FuncNotFound {
            account_addr,
            template_addr,
            func,
        } => {
            w.write_template_addr(template_addr);
            w.write_address(account_addr);
            w.write_string(func);
        }
        RuntimeError::FuncFailed {
            account_addr,
            template_addr,
            func,
            msg,
        } => {
            w.write_template_addr(template_addr);
            w.write_address(account_addr);
            w.write_string(func);
            w.write_string(msg);
        }
        RuntimeError::FuncNotAllowed {
            account_addr,
            template_addr,
            func,
            msg,
        } => {
            w.write_template_addr(template_addr);
            w.write_address(account_addr);
            w.write_string(func);
            w.write_string(msg);
        }
        RuntimeError::FuncInvalidSignature {
            account_addr,
            template_addr,
            func,
        } => {
            w.write_template_addr(template_addr);
            w.write_address(account_addr);
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
        template_addr,
        account_addr,
        msg,
    }
}

fn instantiation_error(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::InstantiationFailed {
        template_addr,
        account_addr,
        msg,
    }
}

fn func_not_found(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let func = decode_func(cursor);

    RuntimeError::FuncNotFound {
        template_addr,
        account_addr,
        func,
    }
}

fn func_failed(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let func = decode_func(cursor);
    let msg = decode_msg(cursor);

    RuntimeError::FuncFailed {
        template_addr,
        account_addr,
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
        template_addr,
        account_addr,
        func,
        msg,
    }
}

fn func_invalid_sig(cursor: &mut Cursor<&[u8]>) -> RuntimeError {
    let template_addr = decode_template_addr(cursor);
    let account_addr = decode_account_addr(cursor);
    let func = decode_func(cursor);

    RuntimeError::FuncInvalidSignature {
        template_addr,
        account_addr,
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
            account_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");

        let err = RuntimeError::InstantiationFailed {
            account_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "do_something".to_string();

        let err = RuntimeError::FuncNotFound {
            account_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "do_something".to_string();
        let msg = "Invalid input".to_string();

        let err = RuntimeError::FuncFailed {
            account_addr,
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
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "init".to_string();
        let msg = "expected a ctor".to_string();

        let err = RuntimeError::FuncNotAllowed {
            account_addr,
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
