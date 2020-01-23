use crate::helpers;

use wasmer_runtime::Ctx as WasmerCtx;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

/// Copies the contents of `wasmer` memory cells under addresses:
/// `mem_offset, mem_offset + 1, .. , mem_offset + count (exclusive)`
/// into `SVM` register
///
/// * `ctx`        - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `mem_idx`    - The source memory index we want to copy from
/// * `mem_offset` - A pointer for the first memory address we want to start copying from
/// * `reg_bits`   - The type of the register (determined by its #bits) we want to copy data to
/// * `reg_idx`    - The destination register we want to load the memory slice into
/// * `count`      - Number of bytes to copy
pub fn mem_to_reg_copy(
    ctx: &mut WasmerCtx,
    mem_idx: i32,
    mem_offset: i32,
    reg_bits: i32,
    reg_idx: i32,
    count: i32,
) {
    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, count);
    let cells = &ctx.memory(mem_idx).view()[start..end];
    let data: Vec<u8> = cells.iter().map(|cell| cell.get()).collect();

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(&data[..]);
}

/// Copies the content of `wasmer` register indexed `src_reg` into `wasmer` memory cells under addresses:
/// `mem_offset, mem_offset + 1, .. , mem_offset + count (exclusive)`
///
/// * `ctx`        - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `reg_bits`   - The type of the register (determined by its #bits) we want to copy data from
/// * `reg_idx`    - The source register index we want to load its content from
/// * `mem_idx`    - The index of the memory we want to copy to
/// * `mem_offset` - A pointer to the first memory address we want to start copying content to
/// * `count`      - Number of bytes to copy
pub fn reg_to_mem_copy(
    ctx: &mut WasmerCtx,
    reg_bits: i32,
    reg_idx: i32,
    mem_idx: i32,
    mem_offset: i32,
    count: i32,
) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let bytes = reg.getn(count as usize);

    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, count);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    for (cell, byte) in cells.iter().zip(bytes) {
        cell.set(*byte);
    }
}

/// Loads from the `SVM` instance's storage a page-slice into the register indexed `dest_reg`
///
/// * `ctx`      - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`     - Page index
/// * `offset`   - Slice starting offset (within the given page)
/// * `reg_bits` - The type of the register (determined by its #bits) we want to copy data to
/// * `reg_idx`  - The destination register index we want to load the page-slice into
/// * `count`    - Number of bytes to read
pub fn storage_read_to_reg(
    ctx: &mut WasmerCtx,
    page_idx: i32,
    page_offset: i32,
    reg_bits: i32,
    reg_idx: i32,
    count: i32,
) {
    let mut storage = helpers::wasmer_data_app_storage(ctx.data);
    let slice = helpers::storage_read_page_slice(&mut storage, page_idx, page_offset, count);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(&slice);
}

/// Loads from the `SVM` instance's storage a page-slice into the memory address given
///
/// * `ctx`        - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`       - Page index
/// * `offset`     - Slice starting offset (within the given page)
/// * `mem_idx`    - The destination memory index we want to copy to
/// * `mem_offset` - The destination memory address to start copying the page-slice into
/// * `count`      - Number of bytes to read
pub fn storage_read_to_mem(
    ctx: &mut WasmerCtx,
    page_idx: i32,
    page_offset: i32,
    mem_idx: i32,
    mem_offset: i32,
    count: i32,
) {
    let mut storage = helpers::wasmer_data_app_storage(ctx.data);
    let mut slice = helpers::storage_read_page_slice(&mut storage, page_idx, page_offset, count);

    if slice.len() == 0 {
        // slice is empty, i.e it doesn't really exist
        // so we fallback to zeros page-slice
        slice.resize(count as usize, 0);
    }

    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, count);
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
/// * `page_idx`    - Destination page
/// * `page_offset` - Destination slice offset
/// * `count`       - Number of bytes to write
pub fn storage_write_from_mem(
    ctx: &mut WasmerCtx,
    mem_idx: i32,
    mem_offset: i32,
    page_idx: i32,
    page_offset: i32,
    count: i32,
) {
    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, count);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    let data = cells.iter().map(|cell| cell.get()).collect::<Vec<u8>>();
    let storage = helpers::wasmer_data_app_storage(ctx.data);

    helpers::storage_write_page_slice(storage, page_idx, page_offset, count, &data);
}

/// Writes into `SVM` storage, a page-slice copied from `SVM wasmer` register
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `reg_bits`    - The type of the register (determined by its #bits) we want to copy data from
/// * `reg_idx`     - Source register to start copying from
/// * `page_idx`    - Destination page
/// * `page_offset` - Destination slice offset
/// * `count`       - Number of bytes to write
pub fn storage_write_from_reg(
    ctx: &mut WasmerCtx,
    reg_bits: i32,
    reg_idx: i32,
    page_idx: i32,
    page_offset: i32,
    count: i32,
) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let data = reg.getn(count as usize);

    helpers::storage_write_page_slice(storage, page_idx, page_offset, count, data);
}

pub fn storage_read_i32_be(
    ctx: &mut WasmerCtx,
    page_idx: i32,
    page_offset: i32,
    count: i32,
) -> u32 {
    assert!(count >= 0 && count <= 4);

    storage_read_int::<BigEndian>(ctx, page_idx, page_offset, count) as u32
}

pub fn storage_read_i32_le(
    ctx: &mut WasmerCtx,
    page_idx: i32,
    page_offset: i32,
    count: i32,
) -> u32 {
    assert!(count >= 0 && count <= 4);

    storage_read_int::<LittleEndian>(ctx, page_idx, page_offset, count) as u32
}

pub fn storage_read_i64_be(
    ctx: &mut WasmerCtx,
    page_idx: i32,
    page_offset: i32,
    count: i32,
) -> u64 {
    assert!(count >= 0 && count <= 8);

    storage_read_int::<BigEndian>(ctx, page_idx, page_offset, count)
}

pub fn storage_read_i64_le(
    ctx: &mut WasmerCtx,
    page_idx: i32,
    page_offset: i32,
    count: i32,
) -> u64 {
    assert!(count >= 0 && count <= 8);

    storage_read_int::<LittleEndian>(ctx, page_idx, page_offset, count)
}

fn storage_read_int<T: ByteOrder>(
    ctx: &mut WasmerCtx,
    page_idx: i32,
    page_offset: i32,
    count: i32,
) -> u64 {
    let mut storage = helpers::wasmer_data_app_storage(ctx.data);
    let buf = helpers::storage_read_page_slice(&mut storage, page_idx, page_offset, count);

    T::read_uint(&buf[..], count as usize)
}

fn rustify_mem_params(mem_idx: i32, mem_offset: i32, count: i32) -> (u32, usize, usize) {
    assert!(mem_idx >= 0);
    assert!(mem_offset >= 0);
    assert!(count >= 0);

    let start = mem_offset as usize;
    let end = start + count as usize;

    (mem_idx as u32, start, end)
}
