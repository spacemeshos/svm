use crate::helpers;

/// Give register `reg_bits:reg_idx`, reads its first 8 bytes and interpretes them as a 64 bit BigEndian number.
pub fn reg_read_be_i64(ctx: &mut wasmer_runtime::Ctx, reg_bits: i32, reg_idx: i32) -> i64 {
    use byteorder::{BigEndian, ByteOrder};

    log::debug!("`reg_read_be_i64` register=`{}:{}`", reg_bits, reg_idx);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    let buf = reg.getn(8);

    BigEndian::read_i64(&buf)
}

/// Give a 64 bits number `value`, stores it under register `reg_bits:reg_idx` in a Big-Endian layout.
pub fn reg_write_be_i64(ctx: &mut wasmer_runtime::Ctx, value: i64, reg_bits: i32, reg_idx: i32) {
    use byteorder::{BigEndian, ByteOrder};

    log::debug!(
        "`reg_write_be_i64` register=`{}:{}`, value={}",
        reg_bits,
        reg_idx,
        value
    );

    let mut buf = [0; 8];
    BigEndian::write_i64(&mut buf, value);

    let reg = helpers::wasmer_data_reg(ctx.data, reg_bits, reg_idx);
    reg.set(&buf);
}
