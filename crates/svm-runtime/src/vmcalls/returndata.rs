use crate::use_gas;
use crate::Context;

/// Signals the host that the data that resides at offset `offset` of length `length`
/// holds the `Returndata` of the executed function.
pub fn set_returndata(ctx: &Context, offset: u32, length: u32) {
    use_gas!("set_returndata", ctx);

    ctx.borrow_mut()
        .set_returndata(offset as usize, length as usize)
}
