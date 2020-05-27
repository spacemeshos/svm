use crate::{helpers, use_gas};

use byteorder::{BigEndian, ByteOrder};
use wasmer_runtime::Ctx as WasmerCtx;

use svm_layout::VarId;

/// Stores memory cells `[mem_ptr, mem_ptr + 1, ..., mem_ptr + 19]` into variable `var_id`.
///
/// # Panics
///
/// Panics if variable `var_id`'s length isn't 20 bytes.
pub fn store160(ctx: &mut WasmerCtx, mem_idx: u32, mem_ptr: u32, var_id: u32) {
    use_gas!("store160", ctx);

    let mem_ptr = mem_ptr as usize;
    let view = &ctx.memory(mem_idx).view::<u8>()[mem_ptr..(mem_ptr + 20)];

    let bytes: Vec<u8> = view.iter().map(|cell| cell.get()).collect();

    let storage = helpers::wasmer_data_app_storage(ctx.data);
    storage.write_var(VarId(var_id), bytes);
}

/// Loads variable `var_id` data into memory cells `[mem_ptr, mem_ptr + 1, ..., mem_ptr + 19]`
///
/// Returns the variable's length.
///
/// # Panics
///
/// Panics if variable `var_id`'s length isn't 20 bytes.
pub fn load160(ctx: &mut WasmerCtx, var_id: u32, mem_idx: u32, mem_ptr: u32) {
    use_gas!("load160", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);

    let bytes = storage.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert_eq!(nbytes, 20);

    let mem_ptr = mem_ptr as usize;
    let view = &ctx.memory(mem_idx).view::<u8>()[mem_ptr..(mem_ptr + 20)];

    for (cell, &byte) in view.iter().zip(bytes.iter()) {
        cell.set(byte);
    }
}

/// Returns the data stored by variable `var_id` as 32-bit integer.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when it consumes more than 32-bit.
pub fn get32(ctx: &mut WasmerCtx, var_id: u32) -> u32 {
    use_gas!("get32", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);

    let bytes = storage.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert!(nbytes <= 4);

    let num = BigEndian::read_uint(&bytes, nbytes);

    debug_assert!(num <= std::u32::MAX as u64);

    num as u32
}

/// Sets the data of variable `var_id` to Big-Endian representation of `value`.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when it consumes more than 32-bit,
/// or when it has not enough bytes to hold `value`.
pub fn set32(ctx: &mut WasmerCtx, var_id: u32, value: u32) {
    use_gas!("set32", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let (_off, nbytes) = storage.var_layout(VarId(var_id));

    assert!(nbytes <= 4);

    let mut buf = vec![0; nbytes as usize];
    BigEndian::write_uint(&mut buf, value as u64, nbytes as usize);

    storage.write_var(VarId(var_id), buf);
}

/// Returns the data stored by variable `var_id` as 64-bit integer.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when it consumes more than 64-bit.
pub fn get64(ctx: &mut WasmerCtx, var_id: u32) -> u64 {
    use_gas!("get64", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);

    let bytes = storage.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert!(nbytes <= 8);

    BigEndian::read_uint(&bytes, nbytes)
}

/// Sets the data of variable `var_id` to Big-Endian representation of `value`.
///
/// # Panics
///
/// Panics when variable `var_id` consumes more than 64-bit,
/// or when it has not enough bytes to hold `value`.
pub fn set64(ctx: &mut WasmerCtx, var_id: u32, value: u64) {
    use_gas!("set64", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let (_off, nbytes) = storage.var_layout(VarId(var_id));

    assert!(nbytes <= 8);

    let mut buf = vec![0; nbytes as usize];
    BigEndian::write_uint(&mut buf, value, nbytes as usize);

    storage.write_var(VarId(var_id), buf);
}

/// Returns the data stored by variable `var_id` as tuple of `(i64, i64, i32)` holding 160-bit in total (20 bytes).
/// Having a method for 20-byte variables is useful since `Address`, which is a very common App's primitive consumes 20 bytes.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when its length isn't equal to 20.
pub fn get160(ctx: &mut WasmerCtx, var_id: u32) -> (u64, u64, u32) {
    use_gas!("get160", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);

    let bytes = storage.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert_eq!(nbytes, 20);

    let a = BigEndian::read_u64(&bytes[0..8]);
    let b = BigEndian::read_u64(&bytes[8..16]);
    let c = BigEndian::read_u32(&bytes[16..20]);

    (a, b, c)
}

/// Sets the data of variable `var_id` to Big-Endian representation of 20-bytes held together by `a`, `b` and `c`.
/// Parameter `a` hold the most-significant 8-bytes, 'c' the least significant 8-bytes and `b` the remaining middle 4-byte.  
///
/// In total `len(a) + len(b) + len(c) = 20` bytes.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when its length isn't equal to 20.
pub fn set160(ctx: &mut WasmerCtx, var_id: u32, a: u64, b: u64, c: u32) {
    use_gas!("set160", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let (_off, nbytes) = storage.var_layout(VarId(var_id));

    assert_eq!(nbytes, 20);

    let mut buf = vec![0; 20];

    BigEndian::write_u64(&mut buf[0..8], a);
    BigEndian::write_u64(&mut buf[8..16], b);
    BigEndian::write_u32(&mut buf[16..20], c);

    storage.write_var(VarId(var_id), buf);
}

/// Returns the data stored by variable `var_id` as tuple of `(i64, i64, i64, i64)` holding 256-bit in total (32 bytes).
/// Having a method for 32-byte variables is useful since `Public-Key`, which is a very common App's primitive consumes 32 bytes.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or whren its length isn't equal to 32.
pub fn get256(ctx: &mut WasmerCtx, var_id: u32) -> (u64, u64, u64, u64) {
    use_gas!("get256", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);

    let bytes = storage.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert_eq!(nbytes, 32);

    let a = BigEndian::read_u64(&bytes[0..8]);
    let b = BigEndian::read_u64(&bytes[8..16]);
    let c = BigEndian::read_u64(&bytes[16..24]);
    let d = BigEndian::read_u64(&bytes[24..32]);

    (a, b, c, d)
}

/// Sets the data of variable `var_id` to Big-Endian representation of 32-bytes held together by `a`, `b`, `c` and `d`.
///
/// In total `len(a) + len(b) + len(c) + len(d) = 32` bytes.
///
/// # Panics
///
/// Panics when variable `var_id` doesn't exist or when its length isn't equal to 20.
pub fn set256(ctx: &mut WasmerCtx, var_id: u32, a: u64, b: u64, c: u64, d: u64) {
    use_gas!("set160", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let (_off, nbytes) = storage.var_layout(VarId(var_id));

    assert_eq!(nbytes, 32);

    let mut buf = vec![0; 32];

    BigEndian::write_u64(&mut buf[0..8], a);
    BigEndian::write_u64(&mut buf[8..16], b);
    BigEndian::write_u64(&mut buf[16..24], c);
    BigEndian::write_u64(&mut buf[24..32], d);

    storage.write_var(VarId(var_id), buf);
}
