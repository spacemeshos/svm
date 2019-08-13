/// Returns a `wasmer` memory view of cells `mem_start, mem_start + 1, .. , mem_start + len` (exclusive)
#[macro_export]
macro_rules! wasmer_ctx_mem_cells {
    ($ctx: expr, $mem_idx: expr, $mem_start: expr, $len: expr) => {{
        let start = $mem_start as usize;
        let end = start + $len as usize;

        /// we must state explicitly that we view each mem cell as a `u8`
        &$ctx.memory($mem_idx as u32).view::<u8>()[start..end]
    }};
}

/// Copies input `data: &[u8]` into `wasmer` memory cells `mem_start, mem_start + 1, .. , mem_start + data.len()` (exclusive)
#[macro_export]
macro_rules! wasmer_ctx_mem_cells_write {
    ($ctx: expr, $mem_idx: expr, $mem_start: expr, $data: expr) => {{
        let cells = wasmer_ctx_mem_cells!($ctx, $mem_idx, $mem_start, $data.len());

        for (cell, byte) in cells.iter().zip($data.iter()) {
            cell.set(*byte);
        }
    }};
}
