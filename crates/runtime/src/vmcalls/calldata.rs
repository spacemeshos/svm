use crate::FuncEnv;

/// Returns the memory offset of where the input `Calldata` starts.
pub fn calldata_offset(env: &FuncEnv) -> i32 {
    calldata(env).0 as i32
}

/// Returns the length of the input `Calldata`
pub fn calldata_len(env: &FuncEnv) -> i32 {
    calldata(env).1 as i32
}

#[inline]
fn calldata(env: &FuncEnv) -> (usize, usize) {
    env.borrow().get_calldata()
}
