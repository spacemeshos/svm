use svm_storage::page::{PageIndex, PageOffset, PageSliceLayout};
use svm_storage::ContractStorage;

/// Copies the contents of `wasmer` memory cells under addresses:
/// `src_mem_ptr, src_mem_ptr + 1, .. , src_mem_ptr + len (exclusive)`
/// into `wasmer` register indexed `dst_reg`
///
/// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `src_mem_idx`  - The source memory index we want to copy from
/// * `src_mem_ptr`  - Pointer for the first memory address we want to start copying from
/// * `len`          - The length of the memory slice we want to copy (in bytes)
/// * `dst_reg_bits` - The type of the register (determined by its #bits) we want to copy data to
/// * `dst_reg_idx`  - The destination register we want to load the memory slice into
pub fn mem_to_reg_copy(
    ctx: &mut wasmer_runtime::Ctx,
    src_mem_idx: i32,
    src_mem_ptr: i32,
    len: i32,
    dst_reg_bits: i32,
    dst_reg_idx: i32,
) {
    let cells = crate::macros::wasmer_ctx_mem_cells_get(ctx, src_mem_idx, src_mem_ptr, len);
    let reg = crate::macros::wasmer_data_reg(ctx.data, dst_reg_bits, dst_reg_idx);
    reg.copy_from_wasmer_mem(cells);
}

/// Copies the content of `wasmer` register indexed `src_reg` into `wasmer` memory cells under addresses:
/// `dst_mem_ptr, dst_mem_ptr + 1, .. , dst_mem_ptr + len (exclusive)`
///
/// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `src_reg_bits` - The type of the register (determined by its #bits) we want to copy data from
/// * `src_reg_idx`  - The source register index we want to load its content from
/// * `len`          - The length of the register content we want to copy into memory (in bytes)
///                    This parameter *must* not be greater than the register capacity
/// * `dst_mem_idx`  - The index of the memory we want to copy to
/// * `dst_mem_ptr`  - Pointer to the first memory address we want to start copying content to
pub fn reg_to_mem_copy(
    ctx: &mut wasmer_runtime::Ctx,
    src_reg_bits: i32,
    src_reg_idx: i32,
    len: i32,
    dst_mem_idx: i32,
    dst_mem_ptr: i32,
) {
    let cells = crate::macros::wasmer_ctx_mem_cells_get(ctx, dst_mem_idx, dst_mem_ptr, len);
    let reg = crate::macros::wasmer_data_reg(ctx.data, src_reg_bits, src_reg_idx);
    reg.copy_to_wasmer_mem(cells);
}

/// Loads from the `svm` instance's storage a page-slice into the register indexed `dest_reg`
///
/// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`         - Page index
/// * `offset`       - Slice starting offset (within the given page)
/// * `len`          - The length of the slice in bytes
/// * `dst_reg_bits` - The type of the register (determined by its #bits) we want to copy data to
/// * `dst_reg_idx`  - The destination register index we want to load the page-slice into
pub fn storage_read_to_reg(
    ctx: &mut wasmer_runtime::Ctx,
    page: i32,
    offset: i32,
    len: i32,
    dst_reg_bits: i32,
    dst_reg_idx: i32,
) {
    let mut storage = crate::macros::wasmer_data_storage(ctx.data);
    let slice = storage_read_page_slice(&mut storage, page, offset, len);

    let reg = crate::macros::wasmer_data_reg(ctx.data, dst_reg_bits, dst_reg_idx);
    reg.set(&slice);
}

/// Loads from the `svm` instance's storage a page-slice into the memory address given
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`        - Page index
/// * `offset`      - Slice starting offset (within the given page)
/// * `len`         - The length of the slice in bytes
/// * `dst_mem_idx` - The destination memory index we want to copy to
/// * `dst_mem_ptr` - The destination memory address to start copying the page-slice into
pub fn storage_read_to_mem(
    ctx: &mut wasmer_runtime::Ctx,
    page: i32,
    offset: i32,
    len: i32,
    dst_mem_idx: i32,
    dst_mem_ptr: i32,
) {
    let mut storage = crate::macros::wasmer_data_storage(ctx.data);
    let mut slice = storage_read_page_slice(&mut storage, page, offset, len);

    if slice.len() == 0 {
        // slice is empty, i.e it doesn't really exist
        // so we fallback to zeros page-slice
        slice.resize(len as usize, 0);
    }

    crate::wasmer_ctx_mem_cells_write!(ctx, dst_mem_idx, dst_mem_ptr, slice);
}

/// Writes into `svm` storage, a page-slice copied from `wasmer` memory
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `src_mem_idx` - The memory index we start to copy from
/// * `src_mem_ptr` - Memory address to start copying from
/// * `len`         - #memory cells to copy
/// * `dst_page`    - Destination page
/// * `dst_offset`  - Destination slice offset
pub fn storage_write_from_mem(
    ctx: &mut wasmer_runtime::Ctx,
    src_mem_idx: i32,
    src_mem_ptr: i32,
    len: i32,
    dst_page: i32,
    dst_offset: i32,
) {
    let cells = crate::macros::wasmer_ctx_mem_cells_get(ctx, src_mem_idx, src_mem_ptr, len);
    let data = cells.iter().map(|cell| cell.get()).collect::<Vec<u8>>();
    let storage = crate::macros::wasmer_data_storage(ctx.data);

    storage_write_page_slice(storage, dst_page, dst_offset, len, &data);
}

/// Writes into `svm` storage, a page-slice copied from `svm wasmer` register
///
/// * `ctx`          - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `src_reg_bits` - The type of the register (determined by its #bits) we want to copy data from
/// * `src_reg_idx`  - Source register to start copying from
/// * `len`          - #register bytes to copy. Must be less than register capacity
/// * `dst_page`     - Destination page
/// * `dst_offset`   - Destination slice offset
pub fn storage_write_from_reg(
    ctx: &mut wasmer_runtime::Ctx,
    src_reg_bits: i32,
    src_reg_idx: i32,
    len: i32,
    dst_page: i32,
    dst_offset: i32,
) {
    let reg = crate::macros::wasmer_data_reg(ctx.data, src_reg_bits, src_reg_idx);
    let storage = crate::macros::wasmer_data_storage(ctx.data);
    let data = reg.getn(len as usize);

    storage_write_page_slice(storage, dst_page, dst_offset, len, &data);
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
