use std::io::{Cursor, Read};

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

    w.push(nlogs as u8);

    for log in logs.iter() {
        let len = log.msg.len();

        assert!(log.msg.len() <= std::u8::MAX as usize);

        // `msg` length
        w.push(len as u8);

        // `msg` blob
        w.extend_from_slice(&log.msg);

        // `msg` code
        w.push(log.code);
    }
}

pub fn decode_logs(cursor: &mut Cursor<&[u8]>) -> Result<Vec<Log>, ParseError> {
    match cursor.read_byte() {
        Ok(nlogs) => {
            let mut logs = Vec::with_capacity(nlogs as usize);

            for _ in (0..nlogs) {
                let log = decode_log(cursor)?;

                logs.push(log);
            }

            Ok(logs)
        }
        Err(..) => Err(ParseError::NotEnoughBytes(Field::LogsCount)),
    }
}

fn decode_log(cursor: &mut Cursor<&[u8]>) -> Result<Log, ParseError> {
    let mut buf = [0; 1];

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::LogMessageLength));
    }

    let len = buf[0];

    let mut msg = Vec::with_capacity(len as usize);
    if cursor.read_exact(&mut msg).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::LogMessage));
    }

    if cursor.read_exact(&mut buf).is_err() {
        return Err(ParseError::NotEnoughBytes(Field::LogCode));
    }
    let code = buf[0];

    let log = Log { msg, code };

    Ok(log)
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
