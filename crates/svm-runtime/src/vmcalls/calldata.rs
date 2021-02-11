use crate::use_gas;
use crate::Context;

/// Returns the memory offset of where the input `Calldata` starts.
pub fn calldata_offset(ctx: &Context) -> i32 {
    use_gas!("calldata_offset", ctx);

    calldata(ctx).0 as i32
}

/// Returns the legth of the input `Calldata`
pub fn calldata_len(ctx: &Context) -> i32 {
    use_gas!("calldata_len", ctx);

    calldata(ctx).1 as i32
}

#[inline]
fn calldata(ctx: &Context) -> (usize, usize) {
    ctx.borrow().get_calldata()
}
