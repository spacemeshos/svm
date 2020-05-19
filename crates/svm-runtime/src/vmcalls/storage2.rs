use crate::{helpers, use_gas};

use byteorder::{BigEndian, ByteOrder};
use wasmer_runtime::Ctx as WasmerCtx;

use svm_layout::VarId;

/// Returns the data stored by variable `var_id` as 64-bit integer.
///
/// # Panics
///
/// Panics when variable `var_id` consumes more than 64-bit.
pub fn get64(ctx: &mut WasmerCtx, var_id: u32) -> u64 {
    use_gas!("get64", ctx);

    let storage2 = helpers::wasmer_data_app_storage2(ctx.data);

    let bytes = storage2.read_var(VarId(var_id));
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

    let storage2 = helpers::wasmer_data_app_storage2(ctx.data);
    let (_off, nbytes) = storage2.var_layout(VarId(var_id));

    assert!(nbytes <= 8);

    let mut buf = vec![0; nbytes as usize];
    BigEndian::write_uint(&mut buf, value, nbytes as usize);

    storage2.write_var(VarId(var_id), buf);
}
