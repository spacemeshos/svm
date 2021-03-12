use std::fmt;

/// A log entry. Logs are generated during executing of transactions.
/// Their main usage is for debugging / testing purposes.
#[derive(PartialEq, Clone)]
pub struct ReceiptLog {
    /// The log message
    pub msg: Vec<u8>,

    /// The log code
    pub code: u8,
}

impl fmt::Debug for ReceiptLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ReceiptLog")
            .field("message", &fmt_msg(self))
            .field("code", &self.code)
            .finish()
    }
}

fn fmt_msg(log: &ReceiptLog) -> String {
    let bytes = log.msg.clone();

    unsafe { String::from_utf8_unchecked(bytes) }
}
