use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Log {
    pub msg: Vec<u8>,

    pub code: u8,
}

impl fmt::Debug for Log {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Log")
            .field("message", &fmt_msg(self))
            .field("code", &self.code)
            .finish()
    }
}

fn fmt_msg(log: &Log) -> String {
    let bytes = log.msg.clone();

    unsafe { String::from_utf8_unchecked(bytes) }
}
