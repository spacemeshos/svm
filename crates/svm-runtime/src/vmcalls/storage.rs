use crate::helpers;

use wasmer_runtime::Ctx as WasmerCtx;

use byteorder::{BigEndian, ByteOrder, LittleEndian};

/// Copies the contents of memory cells under addresses:
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
    mem_idx: u32,
    mem_offset: u32,
    reg_bits: u32,
    reg_idx: u32,
    count: u32,
) {
    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, count);
    let cells = &ctx.memory(mem_idx).view()[start..end];
    let data: Vec<u8> = cells.iter().map(|cell| cell.get()).collect();

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(&data[..]);
}

/// Copies the content of Register indexed `src_reg` into memory cells under addresses:
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
    reg_bits: u32,
    reg_idx: u32,
    mem_idx: u32,
    mem_offset: u32,
    count: u32,
) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let bytes = reg.getn(count as usize);

    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, count);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    for (cell, byte) in cells.iter().zip(bytes) {
        cell.set(*byte);
    }
}

/// Loads from the App's storage a page-slice into the Register `{reg_bits}:{reg_idx}`.
///
/// * `ctx`      - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`     - Page index
/// * `offset`   - Slice starting offset (within the given page)
/// * `reg_bits` - The type of the register (determined by its #bits) we want to copy data to
/// * `reg_idx`  - The destination register index we want to load the page-slice into
/// * `count`    - Number of bytes to read
pub fn storage_read_to_reg(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    reg_bits: u32,
    reg_idx: u32,
    count: u32,
) {
    let mut storage = helpers::wasmer_data_app_storage(ctx.data);
    let slice = helpers::storage_read_page_slice(&mut storage, page_idx, page_offset, count);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(&slice);
}

/// Loads from the App's storage a page-slice into the memory address given.
///
/// * `ctx`        - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page`       - Page index
/// * `offset`     - Slice starting offset (within the given page)
/// * `mem_idx`    - The destination memory index we want to copy to
/// * `mem_offset` - The destination memory address to start copying the page-slice into
/// * `count`      - Number of bytes to read
pub fn storage_read_to_mem(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    mem_idx: u32,
    mem_offset: u32,
    count: u32,
) {
    let mut storage = helpers::wasmer_data_app_storage(ctx.data);
    let mut slice = helpers::storage_read_page_slice(&mut storage, page_idx, page_offset, count);

    if slice.is_empty() {
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

/// Writes into App's storage, a page-slice copied from running App's memory.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `mem_idx`     - The memory index we start to copy from
/// * `mem_offset`  - Memory address to start copying from
/// * `page_idx`    - Destination page
/// * `page_offset` - Destination slice offset
/// * `count`       - Number of bytes to write
pub fn storage_write_from_mem(
    ctx: &mut WasmerCtx,
    mem_idx: u32,
    mem_offset: u32,
    page_idx: u32,
    page_offset: u32,
    count: u32,
) {
    let (mem_idx, start, end) = rustify_mem_params(mem_idx, mem_offset, count);
    let cells = &ctx.memory(mem_idx).view()[start..end];

    let data = cells.iter().map(|cell| cell.get()).collect::<Vec<u8>>();
    let storage = helpers::wasmer_data_app_storage(ctx.data);

    helpers::storage_write_page_slice(storage, page_idx, page_offset, count, &data);
}

/// Writes into `App` storage, a page-slice copied from Register `{reg_bits}:{reg_idx}`.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `reg_bits`    - The type of the register (determined by its #bits) we want to copy data from
/// * `reg_idx`     - Source register to start copying from
/// * `page_idx`    - Destination page
/// * `page_offset` - Destination slice offset
/// * `count`       - Number of bytes to write
pub fn storage_write_from_reg(
    ctx: &mut WasmerCtx,
    reg_bits: u32,
    reg_idx: u32,
    page_idx: u32,
    page_offset: u32,
    count: u32,
) {
    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let data = reg.getn(count as usize);

    helpers::storage_write_page_slice(storage, page_idx, page_offset, count, data);
}

/// Stores into `App`'s storage the integer `n` in Big-Endian order.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page_idx`    - Page index
/// * `page_offset` - Offset for first byte to store.
/// * `n`           - The integer to store.
/// * `nbytes`      - The number of bytes required for storing `n` (`nbytes` <= 4).
pub fn storage_write_i32_be(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    n: u32,
    nbytes: u32,
) {
    storage_write::<BigEndian>(ctx, page_idx, page_offset, n as u64, nbytes);
}

/// Stores into `App`'s storage the integer `n` in Little-Endian order.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page_idx`    - Page index
/// * `page_offset` - Offset for first byte to store.
/// * `n`           - The integer to store.
/// * `nbytes`      - The number of bytes required for storing `n` (`nbytes` <= 4)
pub fn storage_write_i32_le(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    n: u32,
    nbytes: u32,
) {
    storage_write::<LittleEndian>(ctx, page_idx, page_offset, n as u64, nbytes);
}

/// Stores into `App`'s storage the integer `n` in Big-Endian order.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page_idx`    - Page index
/// * `page_offset` - Offset for first byte to store.
/// * `n`           - The integer to store.
/// * `nbytes`      - The number of bytes required for storing `n` (`nbytes` <= 8).
pub fn storage_write_i64_be(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    n: u64,
    nbytes: u32,
) {
    storage_write::<BigEndian>(ctx, page_idx, page_offset, n, nbytes);
}

/// Stores into `App`'s storage the integer `n` in Little-Endian order.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`)
/// * `page_idx`    - Page index
/// * `page_offset` - Offset for first byte to store.
/// * `n`           - The integer to store.
/// * `nbytes`      - The number of bytes required for storing `n` (`nbytes` <= 8).
pub fn storage_write_i64_le(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    n: u64,
    nbytes: u32,
) {
    storage_write::<LittleEndian>(ctx, page_idx, page_offset, n, nbytes);
}

/// Reads `App`'s storage into a 32-bit integer. Integer is interpreted as Big-Endian integer.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`).
/// * `page_idx`    - Page index.
/// * `page_offset` - Offset for first byte to store.
/// * `count`       - The number of bytes required for reading the integer. (`count` <= 4).
pub fn storage_read_i32_be(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    count: u32,
) -> u32 {
    storage_read_int::<BigEndian>(ctx, page_idx, page_offset, count) as u32
}

/// Reads `App`'s storage into a 32-bit integer. Integer is interpreted as Little-Endian integer.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`).
/// * `page_idx`    - Page index.
/// * `page_offset` - Offset for first byte to store.
/// * `count`       - The number of bytes required for reading the integer. (`count` <= 4).
pub fn storage_read_i32_le(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    count: u32,
) -> u32 {
    storage_read_int::<LittleEndian>(ctx, page_idx, page_offset, count) as u32
}

/// Reads `App`'s storage into a 64-bit integer. Integer is interpreted as Big-Endian integer.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`).
/// * `page_idx`    - Page index.
/// * `page_offset` - Offset for first byte to store.
/// * `count`       - The number of bytes required for reading the integer. (`count` <= 4).
pub fn storage_read_i64_be(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    count: u32,
) -> u64 {
    storage_read_int::<BigEndian>(ctx, page_idx, page_offset, count)
}

/// Reads `App`'s storage into a 64-bit integer. Integer is interpreted as Little-Endian integer.
///
/// * `ctx`         - `wasmer` context (holds a `data` field. we use `SvmCtx`).
/// * `page_idx`    - Page index.
/// * `page_offset` - Offset for first byte to store.
/// * `count`       - The number of bytes required for reading the integer. (`count` <= 4).
pub fn storage_read_i64_le(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    count: u32,
) -> u64 {
    storage_read_int::<LittleEndian>(ctx, page_idx, page_offset, count)
}

fn storage_read_int<T: ByteOrder>(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    nbytes: u32,
) -> u64 {
    let mut storage = helpers::wasmer_data_app_storage(ctx.data);
    let buf = helpers::storage_read_page_slice(&mut storage, page_idx, page_offset, nbytes);

    T::read_uint(&buf[..], nbytes as usize)
}

fn storage_write<T: ByteOrder>(
    ctx: &mut WasmerCtx,
    page_idx: u32,
    page_offset: u32,
    n: u64,
    nbytes: u32,
) {
    let mut storage = helpers::wasmer_data_app_storage(ctx.data);

    let mut buf = [0; 8];
    T::write_uint(&mut buf[..], n, nbytes as usize);

    helpers::storage_write_page_slice(&mut storage, page_idx, page_offset, nbytes, &buf[..]);
}

fn rustify_mem_params(mem_idx: u32, mem_offset: u32, count: u32) -> (u32, usize, usize) {
    let start = mem_offset as usize;
    let end = start + count as usize;

    (mem_idx, start, end)
}
