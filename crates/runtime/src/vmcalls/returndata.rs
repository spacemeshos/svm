use crate::FuncEnv;

/// Signals the host that the data that resides at offset `offset` of length `length`
/// holds the `Returndata` of the executed function.
pub fn set_returndata(env: &FuncEnv, offset: u32, length: u32) {
    dbg!("set_returndata (offset = {}, length = {})", offset, length);

    env.borrow_mut()
        .set_returndata(offset as usize, length as usize)
}
