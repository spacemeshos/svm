use std::cell::Cell;
use wasmer_runtime_core::vm::Ctx;

/// Returns a `wasmer` memory view of cells `mem_start, mem_start + 1, ... , mem_start + len` (exclusive)
pub fn wasmer_ctx_mem_cells_get<'mem, 'ctx: 'mem>(
    ctx: &'ctx mut Ctx,
    mem_idx: i32,
    mem_start: i32,
    len: i32,
) -> &'mem [Cell<u8>] {
    assert!(mem_idx >= 0);
    assert!(mem_start >= 0);
    assert!(len >= 0);

    let start = mem_start as usize;
    let end = start + len as usize;

    /// we must state explicitly that we view each mem cell as a `u8`
    let cells = ctx.memory(mem_idx as u32).view::<u8>()[start..end];
    &cells
}

/// Copies input `data: &[u8]` into `wasmer` memory cells `mem_start, mem_start + 1, ... , mem_start + data.len()` (exclusive)
#[macro_export]
macro_rules! wasmer_ctx_mem_cells_write {
    ($ctx: expr, $mem_idx: expr, $mem_start: expr, $data: expr) => {{
        let cells =
            crate::macros::wasmer_ctx_mem_cells_get($ctx, $mem_idx, $mem_start, $data.len() as i32);

        for (cell, byte) in cells.iter().zip($data.iter()) {
            cell.set(*byte);
        }
    }};
}
