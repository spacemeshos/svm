use crate::use_gas;
use crate::Context;

pub fn calldata_offset(ctx: &mut Context) -> i32 {
    use_gas!("calldata_offset", ctx);

    calldata(ctx).0 as i32
}

pub fn calldata_len(ctx: &mut Context) -> i32 {
    use_gas!("calldata_len", ctx);

    calldata(ctx).1 as i32
}

#[inline]
fn calldata(ctx: &mut Context) -> (usize, usize) {
    ctx.borrow().get_calldata()
}
