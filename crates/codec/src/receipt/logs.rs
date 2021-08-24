use svm_types::ReceiptLog;

use crate::{Field, ParseError, ReadExt, WriteExt};

/// ```text                   
/// +----------------+
/// | #logs (1 byte) |
/// +------------------------+----------------------+
/// |  data length (2 bytes) | data (blob of bytes) |  ---> log #1
/// +-----------------------------------------------+
///                       .
///                       .
///                       .
/// +------------------------+----------------------+
/// |  data length (2 bytes) | data (blob of bytes) |  ---> log #N
/// +-----------------------------------------------+
/// ```
pub fn encode_logs(logs: &[ReceiptLog], w: &mut impl WriteExt) {
    let nlogs = logs.len();
    assert!(nlogs <= std::u8::MAX as usize);

    w.write_byte(nlogs as u8);

    for log in logs.iter() {
        let len = log.as_bytes().len();

        assert!(len <= std::u16::MAX as usize);

        // `data` length
        w.write_u16_be(len as u16);

        // `data` blob
        w.write_bytes(log.as_bytes());
    }
}

pub fn decode_logs(cursor: &mut impl ReadExt) -> Result<Vec<ReceiptLog>, ParseError> {
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

fn decode_log(cursor: &mut impl ReadExt) -> Result<ReceiptLog, ParseError> {
    match cursor.read_u16_be() {
        Ok(length) => {
            let data = cursor.read_bytes(length as usize);
            if data.is_err() {
                return Err(ParseError::NotEnoughBytes(Field::LogData));
            };

            let log = ReceiptLog::new(data.unwrap());
            Ok(log)
        }
        Err(..) => Err(ParseError::NotEnoughBytes(Field::LogDataLength)),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

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

        let log = ReceiptLog::new(b"been here".to_vec());
        encode_logs(&[log.clone()], &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = decode_logs(&mut cursor).unwrap();

        assert_eq!(logs, vec![log]);
    }

    #[test]
    fn encode_logs_single_multiple_entries() {
        let mut buf = Vec::new();

        let log1 = ReceiptLog::new(b"been here".to_vec());
        let log2 = ReceiptLog::new(b"been there".to_vec());

        encode_logs(&[log1.clone(), log2.clone()], &mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = decode_logs(&mut cursor).unwrap();

        assert_eq!(logs, vec![log1, log2]);
    }
}
