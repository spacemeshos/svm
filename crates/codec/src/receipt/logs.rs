use std::io::Cursor;

use svm_types::receipt::Log;

use crate::{Field, ParseError, ReadExt, WriteExt};

///                   
/// +-----------------+
/// | #logs (1 byte)  |
/// +-----------------+-------------------------------------------+
/// |  msg length (1 byte) | msg (blob of bytes) | code (1 byte)  |  ---> log #0
/// +-------------------------------------------------------------+
/// |  msg length (1 byte) | msg (blob of bytes) | code (1 byte)  |  ---> log #1
/// +-------------------------------------------------------------+
///                            .
///                            .
///                            .
/// +---------------------------------------+---------------------+
/// |  msg length (1 byte) | msg (blob of bytes) | code (1 byte)  |  ---> log #N
/// +---------------------------------------+---------------------+
///
pub fn encode_logs(logs: &[Log], w: &mut Vec<u8>) {
    let nlogs = logs.len();
    assert!(nlogs <= std::u8::MAX as usize);

    w.write_byte(nlogs as u8);

    for log in logs.iter() {
        let len = log.msg.len();

        assert!(log.msg.len() <= std::u8::MAX as usize);

        // `msg` length
        w.write_byte(len as u8);

        // `msg` blob
        w.write_bytes(&log.msg);

        // `msg` code
        w.write_byte(log.code);
    }
}

pub fn decode_logs(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Log>, ParseError> {
    match cursor.read_byte() {
        Ok(nlogs) => {
            let mut logs = Vec::with_capacity(nlogs as usize);

            for _ in 0..nlogs {
                let log = decode_log(cursor)?;

                logs.push(log);
            }

            Ok(logs)
        }
        Err(..) => Err(ParseError::NotEnoughBytes(Field::LogsCount)),
    }
}

fn decode_log(cursor: &mut Cursor<&[u8]>) -> Result<Log, ParseError> {
    match cursor.read_byte() {
        Ok(length) => {
            let msg = cursor.read_bytes(length as usize);
            if msg.is_err() {
                return Err(ParseError::NotEnoughBytes(Field::LogMessage));
            };

            let code = cursor.read_byte();
            if code.is_err() {
                return Err(ParseError::NotEnoughBytes(Field::LogCode));
            }

            let log = Log {
                msg: msg.unwrap(),
                code: code.unwrap(),
            };

            Ok(log)
        }
        Err(..) => Err(ParseError::NotEnoughBytes(Field::LogMessageLength)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_logs_empty() {
        let mut buf = Vec::new();

        encode_logs(&[], &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = decode_logs(&mut cursor).unwrap();

        assert!(logs.is_empty());
    }

    #[test]
    fn encode_logs_single_entry() {
        let mut buf = Vec::new();

        let log = Log {
            msg: b"been here".to_vec(),
            code: 200,
        };

        encode_logs(&[log.clone()], &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = decode_logs(&mut cursor).unwrap();

        assert_eq!(logs, vec![log]);
    }

    #[test]
    fn encode_logs_single_mulitiple_entries() {
        let mut buf = Vec::new();

        let log1 = Log {
            msg: b"been here".to_vec(),
            code: 200,
        };

        let log2 = Log {
            msg: b"been there".to_vec(),
            code: 201,
        };

        encode_logs(&[log1.clone(), log2.clone()], &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = decode_logs(&mut cursor).unwrap();

        assert_eq!(logs, vec![log1, log2]);
    }
}
