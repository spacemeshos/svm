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

use svm_types::{Address, ReceiptLog, RuntimeError, TemplateAddr};

use crate::{Codec, ParseError, ReadExt, WriteExt};

#[derive(Debug, PartialEq)]
pub struct RuntimeErrorWithLogs {
    pub err: RuntimeError,
    pub logs: Vec<ReceiptLog>,
}

impl RuntimeErrorWithLogs {
    pub fn new<T>(err: RuntimeError, logs: T) -> Self
    where
        T: Into<Vec<ReceiptLog>>,
    {
        Self {
            err,
            logs: logs.into(),
        }
    }
}

impl Codec for RuntimeErrorWithLogs {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        encode_err_type(&self.err, w);

        self.logs.encode(w);

        match &self.err {
            RuntimeError::OOG => (),
            RuntimeError::TemplateNotFound(template) => template.encode(w),
            RuntimeError::AccountNotFound(target) => target.encode(w),
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
                template.encode(w);
                target.encode(w);
                msg.encode(w);
            }
            RuntimeError::FuncNotFound {
                target,
                template,
                func,
            } => {
                template.encode(w);
                target.encode(w);
                func.encode(w);
            }
            RuntimeError::FuncFailed {
                target,
                template,
                func,
                msg,
            } => {
                template.encode(w);
                target.encode(w);
                func.encode(w);
                encode_msg(&msg, w);
            }
            RuntimeError::FuncNotAllowed {
                target,
                template,
                func,
                msg,
            } => {
                template.encode(w);
                target.encode(w);
                func.encode(w);
                encode_msg(&msg, w);
            }
            RuntimeError::FuncInvalidSignature {
                target,
                template,
                func,
            } => {
                template.encode(w);
                target.encode(w);
                func.encode(w);
            }
        };
    }

    fn decode(cursor: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let ty = cursor.read_byte().unwrap();
        let logs = <Vec<ReceiptLog>>::decode(cursor).unwrap();

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

        Ok(Self { err, logs })
    }
}

fn encode_msg(msg: &str, w: &mut impl WriteExt) {
    if msg.len() > 255 {
        let bytes = &msg.as_bytes()[0..255];
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        msg.encode(w);
    } else {
        msg.to_string().encode(w);
    }
}

fn encode_err_type(err: &RuntimeError, w: &mut impl WriteExt) {
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

    w.write_byte(ty);
}

fn oog(_cursor: &mut impl ReadExt) -> RuntimeError {
    RuntimeError::OOG
}

fn template_not_found(cursor: &mut impl ReadExt) -> RuntimeError {
    let template_addr = TemplateAddr::decode(cursor).unwrap();
    RuntimeError::TemplateNotFound(template_addr)
}

fn account_not_found(cursor: &mut impl ReadExt) -> RuntimeError {
    let account = Address::decode(cursor).unwrap();
    RuntimeError::AccountNotFound(account)
}

fn compilation_error(cursor: &mut impl ReadExt) -> RuntimeError {
    let template_addr = TemplateAddr::decode(cursor).unwrap();
    let account_addr = Address::decode(cursor).unwrap();
    let msg = String::decode(cursor).unwrap();

    RuntimeError::CompilationFailed {
        template: template_addr,
        target: account_addr,
        msg,
    }
}

fn instantiation_error(cursor: &mut impl ReadExt) -> RuntimeError {
    let template_addr = TemplateAddr::decode(cursor).unwrap();
    let account_addr = Address::decode(cursor).unwrap();
    let msg = String::decode(cursor).unwrap();

    RuntimeError::InstantiationFailed {
        template: template_addr,
        target: account_addr,
        msg,
    }
}

fn func_not_found(cursor: &mut impl ReadExt) -> RuntimeError {
    let template_addr = TemplateAddr::decode(cursor).unwrap();
    let account_addr = Address::decode(cursor).unwrap();
    let func = String::decode(cursor).unwrap();

    RuntimeError::FuncNotFound {
        template: template_addr,
        target: account_addr,
        func,
    }
}

fn func_failed(cursor: &mut impl ReadExt) -> RuntimeError {
    let template_addr = TemplateAddr::decode(cursor).unwrap();
    let account_addr = Address::decode(cursor).unwrap();
    let func = String::decode(cursor).unwrap();
    let msg = String::decode(cursor).unwrap();

    RuntimeError::FuncFailed {
        template: template_addr,
        target: account_addr,
        func,
        msg,
    }
}

fn func_not_allowed(cursor: &mut impl ReadExt) -> RuntimeError {
    let template_addr = TemplateAddr::decode(cursor).unwrap();
    let account_addr = Address::decode(cursor).unwrap();
    let func = String::decode(cursor).unwrap();
    let msg = String::decode(cursor).unwrap();

    RuntimeError::FuncNotAllowed {
        template: template_addr,
        target: account_addr,
        func,
        msg,
    }
}

fn func_invalid_sig(cursor: &mut impl ReadExt) -> RuntimeError {
    let template_addr = TemplateAddr::decode(cursor).unwrap();
    let account_addr = Address::decode(cursor).unwrap();
    let func = String::decode(cursor).unwrap();

    RuntimeError::FuncInvalidSignature {
        template: template_addr,
        target: account_addr,
        func,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use svm_types::{Address, BytesPrimitive};

    use super::*;
    use crate::codec::test_codec;

    fn test_logs() -> Vec<ReceiptLog> {
        vec![
            ReceiptLog::new(b"Log entry #1".to_vec()),
            ReceiptLog::new(b"Log entry #2".to_vec()),
        ]
    }

    #[test]
    fn decode_receipt_oog() {
        let err = RuntimeError::OOG;

        let mut buf = Vec::new();
        RuntimeErrorWithLogs::new(err, test_logs()).encode(&mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let _decoded = RuntimeErrorWithLogs::decode(&mut cursor);
    }

    #[test]
    fn decode_receipt_template_not_found() {
        let template_addr = TemplateAddr::of("@Template");

        test_codec(RuntimeErrorWithLogs::new(
            RuntimeError::TemplateNotFound(template_addr),
            test_logs(),
        ));
    }

    #[test]
    fn decode_receipt_account_not_found() {
        let account_addr = Address::of("@Account");

        test_codec(RuntimeErrorWithLogs::new(
            RuntimeError::AccountNotFound(account_addr),
            test_logs(),
        ));
    }

    #[test]
    fn decode_receipt_compilation_failed() {
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");

        test_codec(RuntimeErrorWithLogs::new(
            RuntimeError::CompilationFailed {
                target: account_addr,
                template: template_addr,
                msg: "Invalid code".to_string(),
            },
            test_logs(),
        ));
    }

    #[test]
    fn decode_receipt_instantiation_failed() {
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");

        test_codec(RuntimeErrorWithLogs::new(
            RuntimeError::InstantiationFailed {
                target: account_addr,
                template: template_addr,
                msg: "Invalid input".to_string(),
            },
            test_logs(),
        ));
    }

    #[test]
    fn decode_receipt_func_not_found() {
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "do_something".to_string();

        test_codec(RuntimeErrorWithLogs::new(
            RuntimeError::FuncNotFound {
                target: account_addr,
                template: template_addr,
                func,
            },
            test_logs(),
        ));
    }

    #[test]
    fn decode_receipt_func_failed() {
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "do_something".to_string();
        let msg = "Invalid input".to_string();

        test_codec(RuntimeErrorWithLogs::new(
            RuntimeError::FuncFailed {
                target: account_addr,
                template: template_addr,
                func,
                msg,
            },
            test_logs(),
        ));
    }

    #[test]
    fn decode_receipt_func_not_allowed() {
        let template_addr = TemplateAddr::of("@Template");
        let account_addr = Address::of("@Account");
        let func = "init".to_string();
        let msg = "expected a ctor".to_string();

        test_codec(RuntimeErrorWithLogs::new(
            RuntimeError::FuncNotAllowed {
                target: account_addr,
                template: template_addr,
                func,
                msg,
            },
            test_logs(),
        ));
    }
}
