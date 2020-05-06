use crate::{helpers, use_gas};

use byteorder::{BigEndian, ByteOrder};
use wasmer_runtime::Ctx as WasmerCtx;

use svm_storage2::{app::AppStorage as AppStorage2, layout::VarId};

pub fn get64(ctx: &mut WasmerCtx, var_id: u32) -> u64 {
    use_gas!("get64", ctx);

    let storage2 = helpers::wasmer_data_app_storage2(ctx.data);

    let bytes = storage2.read_var(VarId(var_id));
    let nbytes = bytes.len();

    assert!(nbytes <= 8);

    BigEndian::read_uint(&bytes, nbytes)
}

pub fn set64(ctx: &mut WasmerCtx, var_id: u32, value: u64) {
    use_gas!("set64", ctx);

    let mut storage2 = helpers::wasmer_data_app_storage2(ctx.data);

    let bytes: [u8; 8] = value.to_be_bytes();

    let bytes = storage2.write_var(VarId(var_id), bytes.to_vec());
}
