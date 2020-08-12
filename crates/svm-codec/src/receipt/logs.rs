use svm_nibble::{NibbleIter, NibbleWriter};
use svm_types::receipt::Log;

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
pub fn encode_logs(logs: &[Log], w: &mut NibbleWriter) {
    let nlogs = logs.len();
    assert!(nlogs <= std::u8::MAX as usize);

    w.write_bytes(&[nlogs as u8]);

    for log in logs.iter() {
        let len = log.msg.len();
        assert!(log.msg.len() <= std::u8::MAX as usize);

        // `msg` length
        w.write_bytes(&[len as u8]);

        // `msg` blob
        w.write_bytes(&log.msg);

        // `code`
        w.write_bytes(&[log.code]);
    }
}

pub fn decode_logs(iter: &mut NibbleIter) -> Vec<Log> {
    let nlogs = iter.read_byte();

    (0..nlogs)
        .map(|_| {
            let len = iter.read_byte();
            let msg = iter.read_bytes(len as usize);
            let code = iter.read_byte();

            Log { msg, code }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_logs_empty() {
        let mut w = NibbleWriter::new();

        encode_logs(&[], &mut w);

        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let logs = decode_logs(&mut iter);

        assert!(logs.is_empty());
    }

    #[test]
    fn encode_logs_single_entry() {
        let mut w = NibbleWriter::new();

        let log = Log {
            msg: b"been here".to_vec(),
            code: 200,
        };

        encode_logs(&[log.clone()], &mut w);

        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let logs = decode_logs(&mut iter);

        assert_eq!(logs, vec![log]);
    }

    #[test]
    fn encode_logs_single_mulitiple_entries() {
        let mut w = NibbleWriter::new();

        let log1 = Log {
            msg: b"been here".to_vec(),
            code: 200,
        };

        let log2 = Log {
            msg: b"been there".to_vec(),
            code: 201,
        };

        encode_logs(&[log1.clone(), log2.clone()], &mut w);

        let bytes = w.into_bytes();

        let mut iter = NibbleIter::new(&bytes);
        let logs = decode_logs(&mut iter);

        assert_eq!(logs, vec![log1, log2]);
    }
}
