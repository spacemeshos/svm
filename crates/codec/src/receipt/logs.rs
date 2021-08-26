use svm_types::ReceiptLog;

use crate::{Codec, ParseError, ReadExt, WriteExt};

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
impl Codec for Vec<ReceiptLog> {
    type Error = ParseError;

    fn encode(&self, w: &mut impl WriteExt) {
        let nlogs = self.len();
        assert!(nlogs <= std::u8::MAX as usize);

        w.write_byte(nlogs as u8);

        for log in self.iter() {
            let len = log.as_bytes().len();

            assert!(len <= std::u16::MAX as usize);

            // `data` length
            (len as u16).encode(w);

            // `data` blob
            w.write_bytes(log.as_bytes());
        }
    }

    fn decode(cursor: &mut impl ReadExt) -> std::result::Result<Self, Self::Error> {
        let nlogs = cursor.read_byte()?;

        let mut logs = Vec::with_capacity(nlogs as usize);

        for _ in 0..nlogs {
            let log = decode_log(cursor)?;
            logs.push(log);
        }

        Ok(logs)
    }
}

fn decode_log(cursor: &mut impl ReadExt) -> Result<ReceiptLog, ParseError> {
    let length = u16::decode(cursor)?;

    let data = cursor.read_bytes(length as usize)?;

    let log = ReceiptLog::new(data);
    Ok(log)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn encode_logs_empty() {
        let mut buf = Vec::new();

        Vec::<ReceiptLog>::new().encode(&mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = <Vec<ReceiptLog>>::decode(&mut cursor).unwrap();

        assert!(logs.is_empty());
    }

    #[test]
    fn encode_logs_single_entry() {
        let mut buf = Vec::new();

        let log = ReceiptLog::new(b"been here".to_vec());
        vec![log.clone()].encode(&mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = <Vec<ReceiptLog>>::decode(&mut cursor).unwrap();

        assert_eq!(logs, vec![log]);
    }

    #[test]
    fn encode_logs_single_multiple_entries() {
        let mut buf = Vec::new();

        let log1 = ReceiptLog::new(b"been here".to_vec());
        let log2 = ReceiptLog::new(b"been there".to_vec());

        vec![log1.clone(), log2.clone()].encode(&mut buf);

        let mut cursor = Cursor::new(&buf[..]);
        let logs = <Vec<ReceiptLog>>::decode(&mut cursor).unwrap();

        assert_eq!(logs, vec![log1, log2]);
    }
}
