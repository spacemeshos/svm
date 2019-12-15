use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};
use svm_storage::ContractStorage;

use crate::helpers;

/// Copies the contents of `wasmer` memory cells under addresses:
/// `mem_offset, mem_offset + 1, .. , mem_offset + len (exclusive)`
/// into `SVM` register
///
/// * `ctx`        - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `mem_idx`    - The source memory index we want to copy from
/// * `mem_offset` - A pointer for the first memory address we want to start copying from
/// * `len`        - The length of the memory slice we want to copy (in bytes)
/// * `reg_bits`   - The type of the register (determined by its #bits) we want to copy data to
/// * `reg_idx`    - The destination register we want to load the memory slice into
pub fn mem_to_reg_copy(
    ctx: &mut wasmer_runtime::Ctx,
    mem_idx: i32,
    mem_offset: i32,
    len: i32,
    reg_bits: i32,
    reg_idx: i32,
) {
    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, len);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.copy_from_wasmer_mem(cells);
}

/// Copies the content of `wasmer` register indexed `src_reg` into `wasmer` memory cells under addresses:
/// `mem_offset, mem_offset + 1, .. , mem_offset + len (exclusive)`
///
/// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `reg_bits`     - The type of the register (determined by its #bits) we want to copy data from
/// * `reg_idx`      - The source register index we want to load its content from
/// * `len`          - The length of the register content we want to copy into memory (in bytes)
///                    This parameter *must* not be greater than the register capacity
/// * `mem_idx`      - The index of the memory we want to copy to
/// * `mem_offset`   - A pointer to the first memory address we want to start copying content to
pub fn reg_to_mem_copy(
    ctx: &mut wasmer_runtime::Ctx,
    reg_bits: i32,
    reg_idx: i32,
    len: i32,
    mem_idx: i32,
    mem_offset: i32,
) {
    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, len);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.copy_to_wasmer_mem(cells);
}

/// Loads from the `SVM` instance's storage a page-slice into the register indexed `dest_reg`
///
/// * `ctx`      - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`     - Page index
/// * `offset`   - Slice starting offset (within the given page)
/// * `len`      - The length of the slice in bytes
/// * `reg_bits` - The type of the register (determined by its #bits) we want to copy data to
/// * `reg_idx`  - The destination register index we want to load the page-slice into
pub fn storage_read_to_reg(
    ctx: &mut wasmer_runtime::Ctx,
    page: i32,
    offset: i32,
    len: i32,
    reg_bits: i32,
    reg_idx: i32,
) {
    let mut storage = helpers::wasmer_data_storage(ctx.data);
    let slice = storage_read_page_slice(&mut storage, page, offset, len);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(&slice);
}

/// Loads from the `SVM` instance's storage a page-slice into the memory address given
///
/// * `ctx`        - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`       - Page index
/// * `offset`     - Slice starting offset (within the given page)
/// * `len`        - The length of the slice in bytes
/// * `mem_idx`    - The destination memory index we want to copy to
/// * `mem_offset` - The destination memory address to start copying the page-slice into
pub fn storage_read_to_mem(
    ctx: &mut wasmer_runtime::Ctx,
    page: i32,
    offset: i32,
    len: i32,
    mem_idx: i32,
    mem_offset: i32,
) {
    let mut storage = helpers::wasmer_data_storage(ctx.data);
    let mut slice = storage_read_page_slice(&mut storage, page, offset, len);

    if slice.len() == 0 {
        // slice is empty, i.e it doesn't really exist
        // so we fallback to zeros page-slice
        slice.resize(len as usize, 0);
    }

    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, len);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    for (cell, byte) in cells.iter().zip(slice.iter()) {
        cell.set(*byte);
    }
}

/// Writes into `SVM` storage, a page-slice copied from `wasmer` memory
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `mem_idx`     - The memory index we start to copy from
/// * `mem_offset`  - Memory address to start copying from
/// * `len`         - #memory cells to copy
/// * `page_idx`    - Destination page
/// * `page_offset` - Destination slice offset
pub fn storage_write_from_mem(
    ctx: &mut wasmer_runtime::Ctx,
    mem_idx: i32,
    mem_offset: i32,
    len: i32,
    page_idx: i32,
    page_offset: i32,
) {
    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, len);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    let data = cells.iter().map(|cell| cell.get()).collect::<Vec<u8>>();
    let storage = helpers::wasmer_data_storage(ctx.data);

    storage_write_page_slice(storage, page_idx, page_offset, len, &data);
}

/// Writes into `SVM` storage, a page-slice copied from `SVM wasmer` register
///
/// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `reg_bits`     - The type of the register (determined by its #bits) we want to copy data from
/// * `reg_idx`      - Source register to start copying from
/// * `len`          - #register bytes to copy. Must be less than register capacity
/// * `page_idx`     - Destination page
/// * `page_offset`  - Destination slice offset
pub fn storage_write_from_reg(
    ctx: &mut wasmer_runtime::Ctx,
    reg_bits: i32,
    reg_idx: i32,
    len: i32,
    page_idx: i32,
    page_offset: i32,
) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let storage = helpers::wasmer_data_storage(ctx.data);
    let data = reg.getn(len as usize);

    storage_write_page_slice(storage, page_idx, page_offset, len, &data);
}

fn storage_read_page_slice(
    storage: &mut ContractStorage,
    page: i32,
    offset: i32,
    len: i32,
) -> Vec<u8> {
    let layout = page_slice_layout(page, offset, len);
    storage.read_page_slice(&layout)
}

fn storage_write_page_slice(
    storage: &mut ContractStorage,
    page: i32,
    offset: i32,
    len: i32,
    data: &[u8],
) {
    let layout = page_slice_layout(page, offset, len);
    storage.write_page_slice(&layout, data);
}

fn page_slice_layout(page: i32, offset: i32, len: i32) -> PageSliceLayout {
    assert!(page >= 0);
    assert!(offset >= 0);
    assert!(len > 0);

    PageSliceLayout::new(
        PageIndex(page as u32),
        PageOffset(offset as u32),
        len as u32,
    )
}

#[inline(always)]
fn rustify_mem_params(mem_idx: i32, mem_offset: i32, len: i32) -> (u32, usize, usize) {
    assert!(mem_idx >= 0);
    assert!(mem_offset >= 0);
    assert!(len >= 0);

    let start = mem_offset as usize;
    let end = start + len as usize;

    (mem_idx as u32, start, end)
}
