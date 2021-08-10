use svm_types::ReceiptLog;

use crate::FuncEnv;

/// Logs the log entry given in a form of blob (offset and length).
pub fn log(env: &FuncEnv, offset: u32, length: u32) {
    let start = offset as usize;
    let end = start + length as usize;

    // We introduce a new scope to avoid runtime ownership error by the `env`'s internal [`std::cell::RefCell`].
    let bytes = {
        let borrow = env.borrow();
        let memory = borrow.memory();

        memory.view()[start..end]
            .iter()
            .map(|cell| cell.get())
            .collect()
    };

    let log = ReceiptLog::new(bytes);

    env.borrow_mut().logs_mut().push(log);
}
