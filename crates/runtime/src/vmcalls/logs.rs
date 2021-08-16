use svm_types::ReceiptLog;

use crate::FuncEnv;

/// Logs the log entry given.
///
/// It's string message sits in memory starting from offset `msg_ptr` and its length is `msg_len`.
/// The log entry numeric code is given via parameter `code`.
pub fn log(env: &FuncEnv, msg_ptr: u32, msg_len: u32, code: u32) {
    let start = msg_ptr as usize;
    let end = start + msg_len as usize;

    let msg: Vec<u8> = {
        let borrow = env.borrow();
        let memory = borrow.memory();

        memory.view()[start..end]
            .iter()
            .map(|cell| cell.get())
            .collect()
    };

    let log = ReceiptLog {
        msg,
        code: code as u8,
    };

    env.borrow_mut().logs_mut().push(log);
}
