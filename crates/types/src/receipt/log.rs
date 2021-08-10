use std::fmt;

/// A log entry. Logs are generated during executing of transactions.
/// Their main usage is for debugging / testing purposes.
#[derive(PartialEq, Clone)]
pub struct ReceiptLog {
    bytes: Vec<u8>,
}

impl ReceiptLog {
    /// New log entry
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Borrows the underlying bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Debug for ReceiptLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ReceiptLog")
            .field("data", &fmt_msg(self))
            .finish()
    }
}

fn fmt_msg(log: &ReceiptLog) -> String {
    let bytes = log.as_bytes().to_vec();

    unsafe { String::from_utf8_unchecked(bytes) }
}
