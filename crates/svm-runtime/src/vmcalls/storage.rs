use crate::{helpers, use_gas};

use byteorder::{ByteOrder, LittleEndian};
use wasmer_runtime::Ctx as WasmerCtx;

use svm_layout::VarId;

/// Returns the data stored by variable `var_id` as 32-bit integer.
///
/// # Panics
///
/// Panics when variable `var_id` consumes more than 32-bit.
pub fn get32(ctx: &mut WasmerCtx, var_id: u32) -> u32 {
    use_gas!("get32", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);

    let bytes = storage.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert!(nbytes <= 4);

    let num = LittleEndian::read_uint(&bytes, nbytes);

    debug_assert!(num <= std::u32::MAX as u64);

    num as u32
}

/// Sets the data of variable `var_id` to Little-Endian representation of `value`.
///
/// # Panics
///
/// Panics when variable `var_id` consumes more than 32-bit,
/// or when it has not enough bytes to hold `value`.
pub fn set32(ctx: &mut WasmerCtx, var_id: u32, value: u32) {
    use_gas!("set32", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);
    let (_off, nbytes) = storage.var_layout(VarId(var_id));

    assert!(nbytes <= 4);

    let mut buf = vec![0; nbytes as usize];
    LittleEndian::write_uint(&mut buf, value as u64, nbytes as usize);

    storage.write_var(VarId(var_id), buf);
}

/// Returns the data stored by variable `var_id` as 64-bit integer.
///
/// # Panics
///
/// Panics when variable `var_id` consumes more than 64-bit.
pub fn get64(ctx: &mut WasmerCtx, var_id: u32) -> u64 {
    use_gas!("get64", ctx);

    let storage = helpers::wasmer_data_app_storage(ctx.data);

    let bytes = storage.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert!(nbytes <= 8);

    LittleEndian::read_uint(&bytes, nbytes)
}

/// Sets the data of variable `var_id` to Little-Endian representation of `value`.
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
    LittleEndian::write_uint(&mut buf, value, nbytes as usize);

    storage.write_var(VarId(var_id), buf);
}
