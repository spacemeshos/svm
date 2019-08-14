/// When called, injects the code of the `svm wasmer register vmcalls`.
/// The `vmcalls` are functions imported into each running `svm wasmer` instance.
#[macro_export]
macro_rules! include_wasmer_svm_register_vmcalls {
    ($PC: ident) => {
        pub fn reg_read_le_i64(ctx: &mut wasmer_runtime::Ctx, reg_bits: i32, reg_idx: i32) -> i64 {
            use byteorder::{ByteOrder, LittleEndian};

            let reg = wasmer_data_reg!(ctx.data, reg_bits, reg_idx, $PC);
            let buf = reg.getn(8);

            LittleEndian::read_i64(&buf)
        }

        pub fn reg_write_le_i64(
            ctx: &mut wasmer_runtime::Ctx,
            value: i64,
            reg_bits: i32,
            reg_idx: i32,
        ) {
            use byteorder::{ByteOrder, LittleEndian};

            let mut buf = [0; 8];
            LittleEndian::write_i64(&mut buf, value);

            let reg = wasmer_data_reg!(ctx.data, reg_bits, reg_idx, $PC);
            reg.set(&buf);
        }
    };
}
