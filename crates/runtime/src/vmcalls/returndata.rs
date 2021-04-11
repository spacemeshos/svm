use crate::Context;

/// Signals the host that the data that resides at offset `offset` of length `length`
/// holds the `Returndata` of the executed function.
pub fn set_returndata(ctx: &Context, offset: u32, length: u32) {
    dbg!("set_returndata (offset = {}, length = {})", offset, length);

    ctx.borrow_mut()
        .set_returndata(offset as usize, length as usize)
}
