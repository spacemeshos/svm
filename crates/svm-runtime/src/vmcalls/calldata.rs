use crate::Context;
use crate::{helpers, use_gas};

pub fn calldata_ptr(ctx: &mut Context) -> i32 {
    use_gas!("calldata_ptr", ctx);

    calldata(ctx).0 as i32
}

pub fn calldata_len(ctx: &mut Context) -> i32 {
    use_gas!("calldata_len", ctx);

    calldata(ctx).1 as i32
}

#[inline]
fn calldata(ctx: &mut Context) -> (usize, usize) {
    ctx.get_calldata()
}
