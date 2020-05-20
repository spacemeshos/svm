use crate::{helpers, use_gas};

use wasmer_runtime::Ctx as WasmerCtx;

/// Creates a new buffer with id `buf_id` and capacity `cap` for running App.
pub fn buffer_create(ctx: &mut WasmerCtx, buf_id: u32, cap: u32) {
    use_gas!("buffer_create", ctx);

    helpers::buffer_create(ctx.data, buf_id, cap)
}

/// Kills buffer `buf_id` for running `App`.
pub fn buffer_kill(ctx: &mut WasmerCtx, buf_id: u32) {
    use_gas!("buffer_kill", ctx);

    helpers::buffer_kill(ctx.data, buf_id);
}

/// Turns buffer `buf_id` into read-only for running `App`.
pub fn buffer_freeze(ctx: &mut WasmerCtx, buf_id: u32) {
    use_gas!("buffer_freeze", ctx);

    helpers::buffer_freeze(ctx.data, buf_id);
}
